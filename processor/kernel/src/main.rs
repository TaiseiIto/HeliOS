//! # The application processor kernel

#![feature(abi_x86_interrupt)]
#![no_main]
#![no_std]

mod x64;

use core::{
    arch::asm,
    panic::PanicInfo,
};

#[derive(Debug)]
#[repr(packed)]
pub struct Argument {
    ia32_apic_base: x64::msr::ia32::ApicBase,
}

#[no_mangle]
fn main(argument: &'static Argument) {
    let ia32_apic_base: u64 = argument.ia32_apic_base.into();
    unsafe {
        asm!(
            "int 0x80",
            in("eax") ia32_apic_base,
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
