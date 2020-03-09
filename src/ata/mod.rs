pub mod blockdevice;

use obscuro::io::{In, Out};
use obscuro::interrupts::TICKS;
use blockdevice::BlockDevice;

fn wait400_ns(port: u16) {
    unsafe {
        u8::io_in(port);
    }
}

#[repr(packed)]
struct Status {
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

    pub unsafe fn init(&mut self) -> bool {
        if self.is_floating() {
            return false;
        }

        // To use IDENTIFY command, select target drive by sending 0xA0 for the master drive, or
        // 0xB0 for the slave, to the "drive select" IO port
        if self.is_master {
            u8::io_out(self.ports.dev_select, 0xA0);
        } else {
            u8::io_out(self.ports.dev_select, 0xB0);
        }

        // Then set the Sectorcount, LBAlo, LBAmid, and LBAhi IO ports to 0
        u8::io_out(self.ports.sectors, 0);
        u8::io_out(self.ports.lba_low, 0);
        u8::io_out(self.ports.lba_mid, 0);
        u8::io_out(self.ports.lba_high, 0);

        // Then send the IDENTIFY command (0xEC) to the Command IO port.
        u8::io_out(self.ports.cmd, 0xEC);

        // Then read the Status port again. If the value read is 0, the drive does not exist.
        let status_byte = u8::io_in(self.ports.status);
        if status_byte == 0x00 {
            return false;
        }


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
}
