pub trait BlockDev {
    fn read(&mut self, start: usize, data: &[u8]) -> Result<(), &str>;
    fn write(&mut self, start: usize, data: &[u8]) -> Result<(), &str>;
}

pub struct BlockDevice {
    icon: Icon,
}

pub enum Icon {
    Generic,
    Floppy,
    HDD,
}
