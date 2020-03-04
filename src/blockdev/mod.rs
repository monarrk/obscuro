pub struct BlockDevice {
    icon: Icon,
    read: fn(&mut self, start: usize, data: &[u8]) -> Result<(), &str>,
    write: fn(&mut self, start: usize, data: &[u8]) -> Result<(), &str>,
}

pub enum Icon {
    Generic,
    Floppy,
    HDD,
}
