//! # The application processor kernel

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

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
    Argument::get_mut().send_char('A');
    panic!("End of kernel.elf");
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {
        x64::hlt();
    }
}
