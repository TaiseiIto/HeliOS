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
    alloc::{
        collections::BTreeMap,
        vec::Vec,
    },
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
    let heap_start: usize = memory_map
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
    allocator::initialize(heap_start..heap_end);
    {
        let a: Vec<u8> = (u8::MIN..=u8::MAX).collect();
        let b: Vec<u8> = a
            .iter()
            .filter_map(|n| (*n % 2 == 0).then_some(*n))
            .collect();
        let c: Vec<u8> = a
            .iter()
            .filter_map(|n| (*n % 3 == 0).then_some(*n))
            .collect();
        let d: Vec<u8> = a
            .iter()
            .filter_map(|n| (*n % 4 == 0).then_some(*n))
            .collect();
        com2_println!("a = {:#x?}", a);
        com2_println!("b = {:#x?}", a);
        com2_println!("c = {:#x?}", a);
        com2_println!("d = {:#x?}", a);
    }
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

