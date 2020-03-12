pub mod blockdevice;

use core::convert::TryInto;

use obscuro::io::{In, Out};
use obscuro::interrupts::TICKS;
use obscuro::{serial_print, serial_println, print, println};
use obscuro::mem::is_aligned;

use blockdevice::{BlockDevice, Icon};

fn wait400_ns(port: u16) {
    unsafe {
        u8::io_in(port);
    }
}

#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Status {
    /// Indicates an error occurred. Send a new command to clear it (or nuke it with a Software Reset).
    has_error: bool,

    /// Index. Always set to zero.
    index: u8,

    /// Corrected data. Always set to zero.
    corrected_data: bool,

    /// Set when the drive has PIO data to transfer, or is ready to accept PIO data.
    data_request: bool,

    /// Overlapped Mode Service Request.
    service_request: bool,

    /// Drive Fault Error (does not set ERR).
    drive_fault: bool,

    /// Bit is clear when drive is spun down, or after an error. Set otherwise.
    ready: bool,

    /// Indicates the drive is preparing to send/receive data (wait for it to clear). In case of 'hang' (it never clears), do a software reset.
    busy: bool,
}

impl Status {
    fn new() -> Self {
        Self {
            has_error: false,
            index: 0,
            corrected_data: false,
            data_request: false,
            service_request: false,
            drive_fault: false,
            ready: true,
            busy: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Ports {
    data: u16,
    error: u16,
    sectors: u16,
    lba_low: u16,
    lba_mid: u16,
    lba_high: u16,
    dev_select: u16,
    status: u16,
    cmd: u16,
    control: u16,
}

#[derive(Copy, Clone)]
pub struct Device {
    device: BlockDevice,
    block_size: usize,
    sector_count: usize,
    is_master: bool,
    base_port: u16,
    ports: Ports,
    present: bool,
}

impl Device {
    pub fn status(&mut self) -> Status {
        Status::new()
    }

    pub fn is_floating(&mut self) -> bool {
        unsafe { u8::io_in(self.ports.status) == 0xFF }
    }

    pub fn wait_for_err_or_ready(&mut self, timeout: usize) -> Result<(), ()> {
        unsafe {
            let end = TICKS + timeout;
            while TICKS < end {
                let stat = self.status();
                if stat.has_error {
                    return Err(());
                }
                if stat.ready {
                    return Ok(());
                }
            }
        }
        Ok(())
    }

    pub unsafe fn setup_parameters(&mut self, lba: u32, blockcount: u8) {
        if self.is_master {
            u8::io_out(self.ports.dev_select, 0xE0);
        } else {
            u8::io_out(self.ports.dev_select, 0xF0);
        }

        u8::io_out(self.ports.sectors, blockcount);
        u8::io_out(self.ports.lba_low, lba as u8);
        u8::io_out(self.ports.lba_mid, (lba >> 8) as u8);
        u8::io_out(self.ports.lba_high, (lba >> 16) as u8);
    }

    pub unsafe fn init(&mut self) -> bool {
        if self.is_floating() {
            return false;
        }

        // To use IDENTIFY command, select target drive by sending 0xA0 for the master drive, or
        // 0xB0 for the slave, to the "drive select" IO port
        serial_print!("Sending IDENTIFY command...");
        if self.is_master {
            serial_print!("sending 0xA0...");
            u8::io_out(self.ports.dev_select, 0xA0);
        } else {
            serial_print!("sending 0xB0...");
            u8::io_out(self.ports.dev_select, 0xB0);
        }
        serial_println!("OK");

        // Then set the Sectorcount, LBAlo, LBAmid, and LBAhi IO ports to 0
        serial_print!("Setting sectorcount and LBAs to 0...");
        u8::io_out(self.ports.sectors, 0);
        u8::io_out(self.ports.lba_low, 0);
        u8::io_out(self.ports.lba_mid, 0);
        u8::io_out(self.ports.lba_high, 0);
        serial_println!("OK");

        // Then send the IDENTIFY command (0xEC) to the Command IO port.
        serial_print!("Sending IDENTIFY command...");
        // TODO Double faults on QEMU...
        u8::io_out(self.ports.cmd, 0xEC);
        serial_println!("OK");

        // Then read the Status port again. If the value read is 0, the drive does not exist.
        serial_print!("Reading status port...");
        let status_byte = u8::io_in(self.ports.status);
        if status_byte == 0x00 {
            return false;
        }
        serial_println!("OK");

        // For any other value: poll the Status port (0x1F7) until bit 7 (BSY, value =
        // 0x80)
        // clears. Because of some ATAPI drives that do not follow spec, at this
        // point you
        // need to check the LBAmid and LBAhi ports (0x1F4 and 0x1F5) to
        // see if they are
        // non-zero. If so, the drive is not ATA, and you should
        // stop polling. Otherwise,
        // continue polling one of the Status ports
        // until bit 3 (DRQ, value = 8) sets,
        // or until bit 0 (ERR, value = 1) sets.
        while self.status().busy { }

        if (u8::io_in(self.ports.lba_mid) != 0) || (u8::io_in(self.ports.lba_high) != 0) {
            return false;
        }

        match &self.wait_for_err_or_ready(150) {
            Ok(_) => {},
            Err(_) => return false,
        }

        // At that point, if ERR is clear, the data is ready to read from the Data port
        // (0x1F0).
        // Read 256 16-bit values, and store them.
        let ata_data: [u16; 256] = [0; 256];
        for mut w in ata_data.iter() {
            w = &u16::io_in(self.ports.data);
        }

        self.sector_count = ((ata_data[61] as usize) << 16) | ata_data[60] as usize;

        true
    }

    pub fn read_blocks(&mut self, lba: u32, buffer: &[u8]) -> Result<(), &str> {
        if !self.present {
            return Err("Device not present");
        }

        if !is_aligned(buffer.len(), self.block_size) {
            return Err("Data not aligned");
        }

        let block_count = (buffer.len() / self.block_size) as u8;

        if (lba + block_count as u32) > self.sector_count.try_into().unwrap() {
            return Err("Address is not on device");
        }

        unsafe {
            self.setup_parameters(lba, block_count);
            u8::io_out(self.ports.cmd, 0x20);
        }

        let mut block: usize = 0;
        while block < block_count.try_into().unwrap() {
            self.wait_for_err_or_ready(150).unwrap();
            
            let words: &[u16; 256] = &[0; 256];

            unsafe {
                for mut w in words.iter() {
                    w = &u16::io_in(self.ports.data);
                }
            }

            // Forgive me lord, for I have sinned
            unsafe {
                //core::intrinsics::copy_nonoverlapping::<u32>(core::mem::transmute::<[u16; 256], *const u32>(*words), (core::mem::transmute::<&[u8], usize>(buffer) + self.block_size * block) as *mut u32, self.block_size);
            }
            
            block += 1;
        }

        Ok(())
    }
}

struct PortConfig {
    port: u16,
    is_master: bool,
}

static mut DEVS: [Option<Device>; 8] = [None; 8];
static mut BLOCKDEVS: [Option<BlockDevice>; 8] = [None; 8];

pub unsafe fn init() -> Result<&'static [Option<BlockDevice>], ()> {
    let baseports: [PortConfig; 8] = [
        PortConfig { port: 0x1F0, is_master: true },
        PortConfig { port: 0x1F0, is_master: false },
        PortConfig { port: 0x170, is_master: true },
        PortConfig { port: 0x170, is_master: false },
        PortConfig { port: 0x1E8, is_master: true },
        PortConfig { port: 0x1E8, is_master: false },
        PortConfig { port: 0x168, is_master: true },
        PortConfig { port: 0x168, is_master: false },
    ];

    let mut dev_count: usize = 0;
    for i in 0..baseports.len() {
        serial_println!("Initializing device {}", i);
        let baseport = baseports[i].port;
        DEVS[i] = Some(Device {
            device: BlockDevice {
                icon: Icon::HDD,
            },
            block_size: 512,
            base_port: baseports[i].port,
            is_master: baseports[i].is_master,
            ports: Ports {
                data: baseport + 0,
                error: baseport + 1,
                sectors: baseport + 2,
                lba_low: baseport + 3,
                lba_mid: baseport + 4,
                lba_high: baseport + 5,
                dev_select: baseport + 6,
                status: baseport + 7,
                cmd: baseport + 7,
                control: baseport + 518,
            },
            sector_count: 0,
            present: false,
        });

        serial_print!("Unwrapping device...");
        let mut dev = &mut DEVS[i].unwrap();
        serial_println!("OK");

        serial_print!("Initing device...");
        dev.present = dev.init();
        serial_println!("OK");

        if dev.present {
            serial_println!("Found device");
            BLOCKDEVS[dev_count] = Some(dev.device);
            dev_count += 1;
        } else {
            serial_println!("No device present");
        }
    }
    serial_println!("Finished initializing {} devices", dev_count);
    println!("Initialized {} devices", dev_count);

    let list: &'static [Option<BlockDevice>] = &BLOCKDEVS[..dev_count];

    return Ok(list);
}
