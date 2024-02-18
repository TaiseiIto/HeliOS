//! # The kernel

#![no_main]
#![no_std]

extern crate alloc;

mod allocator;
mod efi;
mod interrupt;
mod memory;
mod rs232c;
mod x64;

use {
    alloc::collections::BTreeMap,
    core::{
        ops::Range,
        panic::PanicInfo,
    },
};

#[derive(Debug)]
pub struct Argument<'a> {
    com2: &'a mut rs232c::Com,
    cpuid: Option<x64::Cpuid>,
    efi_system_table: &'a mut efi::SystemTable<'a>,
    #[allow(dead_code)]
    fonts: BTreeMap<usize, efi::Font<'a>>,
    #[allow(dead_code)]
    gdt: memory::segment::descriptor::Table,
    #[allow(dead_code)]
    graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
    heap_start: usize,
    #[allow(dead_code)]
    idt: interrupt::descriptor::Table,
    memory_map: efi::memory::Map,
    my_processor_number: Option<usize>,
    processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation>,
    paging: memory::Paging,
}

#[no_mangle]
fn main(argument: &'static mut Argument<'static>) {
    let Argument {
        com2,
        cpuid,
        efi_system_table,
        fonts: _,
        gdt: _,
        graphics_output_protocol: _,
        heap_start,
        idt: _,
        memory_map,
        my_processor_number,
        processor_informations,
        paging,
    } = argument;
    efi_system_table.set();
    rs232c::set_com2(com2);
    allocator::initialize(paging, memory_map, *heap_start);
    com2_println!("cpuid = {:#x?}", cpuid);
    com2_println!("my_processor_number = {:#x?}", my_processor_number);
    com2_println!("processor_informations = {:#x?}", processor_informations);
    let task_register = x64::task::Register::get();
    com2_println!("task_register = {:#x?}", task_register);
    let higher_half_range: Range<u128> = paging.higher_half_range();
    let interrupt_stack_floor: usize = ((higher_half_range.start + (*heap_start as u128)) / 2) as usize;
    com2_println!("interrupt_stack_floor = {:#x?}", interrupt_stack_floor);
    let interrupt_stack_pages: usize = 0x10;
    let interrupt_stack = memory::Stack::new(paging, interrupt_stack_floor, interrupt_stack_pages);
    com2_println!("interrupt_stack = {:#x?}", interrupt_stack);
    efi::SystemTable::get().shutdown();
    panic!("End of kernel.elf");
}

/// # A panic handler of the kernel
#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    com2_println!("KERNEL PANIC!!!");
    com2_println!("{}", panic);
    loop {
        x64::hlt();
    }
}

