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
    cpuid: Option<x64::Cpuid>,
}

#[no_mangle]
fn main(argument: &'static mut Argument<'static>) {
    let Argument {
        com2,
        cpuid,
    } = argument;
    rs232c::set_com2(com2);
    com2_println!("cpuid = {:#x?}", cpuid);
    panic!("End of kernel.elf");
}

/// # A panic handler of the kernel
#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    com2_println!("KERNEL PANIC!!!");
    com2_println!("{}", panic);
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

