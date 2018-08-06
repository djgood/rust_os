#![feature(panic_implementation)]
#![feature(exclusive_range_pattern)]
#![no_std]

// The Rust test framework injects it's own main
#![cfg_attr(not(test), no_main)]
// During testing, silence warnings generated from our self defined _start 
#![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]

#[macro_use]
extern crate lazy_static;

extern crate bootloader_precompiled;
extern crate volatile;
extern crate spin;

#[cfg(test)]
extern crate std;
#[cfg(test)]
extern crate array_init;

use core::panic::PanicInfo;

#[macro_use]
mod vga_buffer;

// This function is called on panic.
// It is ignored during testing since tests are compiled for the host machine,
// with the std library (which has it's own panic_implementation)
#[cfg(not(test))]
#[panic_implementation]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))] // The Rust test framework injects it's own _start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Panic at the disco");
}

