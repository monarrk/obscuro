pub mod blockdevice;

use obscuro::io::In;
use blockdevice::BlockDevice;

fn wait400NS(port: u16) {
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
}
