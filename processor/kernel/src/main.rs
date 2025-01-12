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
mod syscall;
mod task;
mod x64;

pub use argument::Argument;

use {
    alloc::{
        boxed::Box,
        vec::Vec,
    },
    core::{
        ops::Range,
        panic::PanicInfo,
    },
};

const PRIVILEGE_LEVEL: u8 = 0;

#[no_mangle]
fn main(argument: &'static Argument<'static>) {
    x64::cli();
    Argument::set(argument.clone());
    Argument::get_mut().boot_complete();
    memory::initialize(Argument::get().heap_range());
    bsp_println!("Hello, World!");
    bsp_println!("argument = {:#x?}", Argument::get());
    x64::Cpuid::set();
    let cpuid: &x64::Cpuid = x64::Cpuid::get();
    let mut paging = memory::Paging::get(cpuid);
    paging.set();
    // Initialize GDT.
    let mut gdt = memory::segment::descriptor::table::Controller::new();
    // Initialize IDT.
    let _idt = interrupt::descriptor::table::Controller::new(&mut gdt, &mut paging);
    // Initialize syscall.
    syscall::initialize(cpuid, gdt.kernel_code_segment_selector(), gdt.kernel_data_segment_selector(), gdt.application_code_segment_selector(), gdt.application_data_segment_selector());
    // Initialize a current task.
    task::Controller::set_current();
    task::Controller::get_current_mut()
        .unwrap()
        .sti();
    let mut ia32_apic_base = x64::msr::ia32::ApicBase::get(cpuid).unwrap();
    bsp_println!("ia32_apic_base = {:#x?}", ia32_apic_base);
    let local_apic_registers: &mut interrupt::apic::local::Registers = ia32_apic_base.registers_mut();
    let focus_processor_checking: bool = true;
    let eoi_broadcast: bool = true;
    local_apic_registers.enable_spurious_interrupt(focus_processor_checking, eoi_broadcast, interrupt::SPURIOUS_INTERRUPT);
    bsp_println!("local_apic_registers = {:#x?}", local_apic_registers);
    // Tell the BSP initialication completion.
    Argument::get_mut().initialized();
    // Event loop.
    loop {
        match interrupt::Event::pop() {
            Some(event) => event.process(),
            None => x64::hlt(),
        }
    }
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    bsp_println!("APPLICATION PROCESSOR KERNEL PANIC!!!");
    bsp_println!("{}", panic);
    loop {
        x64::hlt();
    }
}

