pub trait Out {
    unsafe fn io_out(self, port: u16, value: Self);
}

pub trait In {
    unsafe fn io_in(port: u16) -> Self;
}

impl Out for u8 {
    unsafe fn io_out(self, port: u16, value: u8) {
        asm!("outb %al, %dx" : : "{al}a" (value), "{dx}Nd" (port));
    }
}

impl Out for u16 {
    unsafe fn io_out(self, port: u16, value: u16) {
        asm!("outw %ax, %dx" : : "{dx}" (port), "{ax}" (value));
    }
}

impl Out for u32 {
    unsafe fn io_out(self, port: u16, value: u32) {
        asm!("outl %eax, %dx" : : "{dx}" (port), "{eax}" (value));
    }
}

impl In for u8 {
    unsafe fn io_in(port: u16) -> u8 {
        let ret: u8;
        asm!(
            "inb %dx, %al"
            : "={al}" (ret)
            : "{dx}" (port)
        );
        ret
    }
}

impl In for u16 {
    unsafe fn io_in(port: u16) -> u16 {
        let ret: u16;
        asm!(
            "inw %dx, %ax"
            : "={ax}" (ret)
            : "{dx}" (port)
        );
        ret
    }
}

impl In for u32 {
    unsafe fn io_in(port: u16) -> u32 {
        let ret: u32;
        asm!(
            "inb  %dx, %al"
            : "={al}" (ret)
            : "{dx}" (port)
        );
        ret
    }
}
