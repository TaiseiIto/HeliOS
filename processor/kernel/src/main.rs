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
    // Prohibit interruptions.
    x64::cli();
    // Set argument from the boot strap processor.
    Argument::set(argument.clone());
    // Report the boot strap processor to complete boot.
    Argument::get_mut().boot_complete();
    // Initialize heap memory.
    memory::initialize(Argument::get().heap_range());
    bsp_println!("Hello, World!");
    bsp_println!("argument = {:#x?}", Argument::get());
    // Initialize CPUID.
    x64::Cpuid::set();
    let cpuid: &x64::Cpuid = x64::Cpuid::get();
    // Initialize paging.
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
    // Allow interruptions.
    task::Controller::get_current_mut()
        .unwrap()
        .sti();
    // Set APIC.
    let mut ia32_apic_base = x64::msr::ia32::ApicBase::get(cpuid).unwrap();
    let local_apic_registers = interrupt::apic::local::Registers::initialize(&mut ia32_apic_base);
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

