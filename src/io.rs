// Basically a direct port of https://github.com/MasterQ32/RetrOS/blob/master/kernel/src/io.zig

trait Out {
    unsafe fn io_out(self, port: u16);
}

trait In {
    unsafe fn io_in(port: u16) -> Self;
}

impl Out for u8 {
    unsafe fn io_out(self, port: u16) {
        asm!("outb %0,%1" : : "a" (port), "dN" (self));
    }
}

impl Out for u16 {
    unsafe fn io_out(self, port: u16) {
        asm!("outw %data, %port" : : "dx" (port), "{ax}" (self));
    }
}

impl Out for u32 {
    unsafe fn io_out(self, port: u16) {
        asm!("outl %data, %port" : : "{dx}" (port), "{eax}" (self));
    }
}

impl In for u8 {
    unsafe fn io_in(port: u16) -> u8 {
        let ret: u8;
        asm!(
            "inb %port, %ret"
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
            "inw %port, %ret"
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
            "inb  %port, %ret"
            : "={al}" (ret)
            : "{dx}" (port)
        );
        ret
    }
}
