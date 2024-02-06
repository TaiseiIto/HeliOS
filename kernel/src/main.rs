//! # The kernel

#![no_main]
#![no_std]

extern crate alloc;

mod allocator;
mod rs232c;
mod x64;

use core::{
    arch::asm,
    panic::PanicInfo,
};

#[derive(Debug)]
pub struct Argument<'a> {
    com2: &'a mut rs232c::Com,
}

#[no_mangle]
fn main(argument: &'static mut Argument<'static>) {
    let Argument {
        com2
    } = argument;
    rs232c::set_com2(com2);
    com2_println!("Hello, kernel.elf!");
}

/// # A panic handler of the kernel
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

