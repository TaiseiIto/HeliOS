//! The kernel

#![no_main]
#![no_std]

use core::{
    arch::asm,
    panic::PanicInfo,
};

#[no_mangle]
fn main() {
    panic!("Kernel Panic!");
}

/// # A panic handler of the kernel
#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

