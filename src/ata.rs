mod blockdev;

use blockdev::BlockDevice;

#[repr(packed)]
struct Status {
    /// Indicates an error occurred. Send a new command to clear it (or nuke it with a Software Reset).
    hasError: bool,

    /// Index. Always set to zero.
    index: u8 = 0,

    /// Corrected data. Always set to zero.
    correctedData: bool = 0,

    /// Set when the drive has PIO data to transfer, or is ready to accept PIO data.
    dataRequest: bool,

    /// Overlapped Mode Service Request.
    serviceRequest: bool,

    /// Drive Fault Error (does not set ERR).
    driveFault: bool,

    /// Bit is clear when drive is spun down, or after an error. Set otherwise.
    ready: bool,

    /// Indicates the drive is preparing to send/receive data (wait for it to clear). In case of 'hang' (it never clears), do a software reset.
    busy: bool,
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
        
    }
}
