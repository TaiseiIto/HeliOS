//! # The application processor kernel

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

mod interrupt;
mod memory;
mod x64;

use core::{
    arch::asm,
    panic::PanicInfo,
};

#[derive(Clone, Debug)]
#[repr(packed)]
pub struct Argument {
    ia32_apic_base: x64::msr::ia32::ApicBase,
}

#[no_mangle]
fn main(argument: &'static Argument) {
    let mut argument: Argument = argument.clone();
    let mut ia32_apic_base: x64::msr::ia32::ApicBase = argument.ia32_apic_base;
    ia32_apic_base.enable();
    let local_apic_id: u8 = ia32_apic_base
        .registers()
        .apic_id();
    unsafe {
        asm!(
            "syscall",
            in("al") local_apic_id,
        );
    }
    panic!("End of kernel.elf");
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {
        x64::hlt();
    }
}
