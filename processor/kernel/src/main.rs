//! # The application processor kernel

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

extern crate alloc;

mod allocator;
mod argument;
mod interrupt;
mod memory;
mod x64;
mod processor;

pub use argument::Argument;

use core::{
    arch::asm,
    panic::PanicInfo,
};

#[no_mangle]
fn main(argument: &'static Argument) {
    Argument::set(argument.clone());
    Argument::get_mut().boot_complete();
    bsp_println!("Hello, World!");
    panic!("End of kernel.elf");
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    bsp_println!("APPLICATION PROCESSOR KERNEL PANIC!!!");
    bsp_println!("{}", panic);
    Argument::get_mut().kernel_complete();
    loop {
        x64::hlt();
    }
}

