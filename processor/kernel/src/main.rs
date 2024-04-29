//! # The application processor kernel

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

mod x64;

use core::panic::PanicInfo;

#[no_mangle]
fn main() {
    panic!("End of kernel.elf");
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {
        x64::hlt();
    }
}
