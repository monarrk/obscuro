#[derive(Copy, Clone)]
pub struct BlockDevice {
    pub icon: Icon,
}

#[derive(Copy, Clone)]
pub enum Icon {
    Generic,
    Floppy,
    HDD,
}
