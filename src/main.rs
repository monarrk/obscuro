#![no_std]
#![no_main]
#![feature(asm)]

mod terminal;
mod serial;
mod io;
mod ata;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("KERNEL PANIC!: {}", _info);
    println!("KERNEL PANIC!: {}", _info);
    obscuro::hlt_loop();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("\nStarting obscuro...");
    println!("Running Obscuro v0.0.3\n\nWelcome!");
    obscuro::init();
    match unsafe { ata::init() } {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to init ATA driver!");
            serial_println!("Failed to init ATA driver!");
        }
    };

    println!("It didn't crash (yet)! Ain't that neat?");
    print!("\n\n$> ");
    obscuro::hlt_loop();
}
