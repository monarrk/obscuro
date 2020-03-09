#![no_std]
#![feature(abi_x86_interrupt)]
#![feature(lang_items)]
#![feature(asm)]

pub mod serial;
pub mod terminal;
pub mod interrupts;
pub mod gdt;
pub mod io;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}


