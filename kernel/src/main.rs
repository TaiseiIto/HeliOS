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
        arch::asm,
        panic::PanicInfo,
    },
};

#[derive(Debug)]
pub struct Argument<'a> {
    com2: &'a mut rs232c::Com,
    cpuid: Option<x64::Cpuid>,
    efi_system_table: &'a mut efi::SystemTable<'a>,
    fonts: BTreeMap<usize, efi::Font<'a>>,
    gdt: memory::segment::descriptor::Table,
    graphics_output_protocol: &'a efi::graphics_output::Protocol<'a>,
    heap_end: usize,
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
        fonts,
        gdt,
        graphics_output_protocol,
        heap_end,
        idt,
        memory_map,
        my_processor_number,
        processor_informations,
        paging,
    } = argument;
    efi_system_table.set();
    rs232c::set_com2(com2);
    com2_println!("cpuid = {:#x?}", cpuid);
    let heap_end: usize = *heap_end;
    com2_println!("heap_end = {:#x?}", heap_end);
    let available_heap_start: usize = memory_map
        .iter()
        .filter(|memory_descriptor| memory_descriptor.is_available())
        .flat_map(|memory_descriptor| memory_descriptor
            .physical_range()
            .step_by(memory::PAGE_SIZE))
        .enumerate()
        .map(|(index, paddr)| {
            let vaddr: usize = heap_end - (index + 1) * memory::PAGE_SIZE;
            let present: bool = true;
            let writable: bool = true;
            let executable: bool = false;
            paging.set_page(vaddr, paddr, present, writable, executable);
            vaddr
        })
        .min()
        .unwrap();
    com2_println!("available_heap_start = {:#x?}", available_heap_start);
    let available_heap_size: usize = heap_end - available_heap_start;
    com2_println!("available_heap_size = {:#x?}", available_heap_size);
    let heap_size: usize = available_heap_size.next_power_of_two();
    com2_println!("heap_size = {:#x?}", heap_size);
    let heap_start: usize = heap_end - heap_size;
    com2_println!("heap_start = {:#x?}", heap_start);
    com2_println!("my_processor_number = {:#x?}", my_processor_number);
    com2_println!("processor_informations = {:#x?}", processor_informations);
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

