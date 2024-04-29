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

#[derive(Debug)]
#[repr(packed)]
pub struct Argument {
    ia32_apic_base: x64::msr::ia32::ApicBase,
}

#[no_mangle]
fn main(argument: &'static mut Argument) {
    let mut ia32_apic_base: x64::msr::ia32::ApicBase = argument.ia32_apic_base;
    let local_apic_registers: &mut interrupt::apic::local::Registers = ia32_apic_base.registers_mut();
    let local_apic_id: u8 = local_apic_registers.apic_id();
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
