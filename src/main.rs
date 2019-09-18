#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

mod terminal;
mod serial;

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
    println!("Running Obscuro v0.0.1\n\nWelcome!");
    obscuro::init();

    println!("It didn't crash (yet)! Isn't Monarrk such a smarty pants?");
    obscuro::hlt_loop();
}
