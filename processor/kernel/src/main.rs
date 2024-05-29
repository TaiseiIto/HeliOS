//! # The application processor kernel

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

extern crate alloc;

mod argument;
mod interrupt;
mod memory;
mod processor;
mod sync;
mod task;
mod x64;

pub use argument::Argument;

use core::{
    arch::asm,
    panic::PanicInfo,
};

#[no_mangle]
fn main(argument: &'static Argument<'static>) {
    Argument::set(argument.clone());
    Argument::get_mut().boot_complete();
    memory::initialize(Argument::get().heap_range());
    bsp_println!("Hello, World!");
    bsp_println!("argument = {:#x?}", Argument::get());
    let cpuid: x64::Cpuid = x64::Cpuid::get().unwrap();
    bsp_println!("cpuid = {:#x?}", cpuid);
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

