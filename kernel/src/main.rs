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
        idt,
        memory_map,
        my_processor_number,
        processor_informations,
        paging,
    } = argument;
    efi_system_table.set();
    rs232c::set_com2(com2);
    com2_println!("cpuid = {:#x?}", cpuid);
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

