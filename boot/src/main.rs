//! # The bootloader

#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

extern crate alloc;

mod processor;
mod efi;
mod elf;
mod kernel;
mod memory;
mod rs232c;
mod x64;

use {
    alloc::{
        boxed::Box,
        collections::BTreeMap,
        vec::Vec,
    },
    core::{
        panic::PanicInfo,
        ops::Range,
    },
};

include!(concat!(env!("OUT_DIR"), "/constants.rs"));

/// # The entry point of the OS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.1 UEFI Image Entry Point
#[no_mangle]
fn efi_main(image_handle: efi::Handle, system_table: &'static mut efi::SystemTable<'static>) -> efi::Status {
    system_table.set();
    // Allocate pages requested to be allocated at specific physical address preferentially.
    let processor_boot_loader_pages: usize = (PROCESSOR_BOOT_LOADER_STACK_FLOOR - PROCESSOR_BOOT_LOADER_BASE) / memory::page::SIZE;
    let processor_boot_loader_pages: Range<efi::memory::PhysicalAddress> = efi::SystemTable::get()
        .allocate_specific_pages(PROCESSOR_BOOT_LOADER_BASE, processor_boot_loader_pages)
        .unwrap();
    efi_println!("Hello, World!");
    let font_protocol = efi::font::Protocol::get();
    let fonts: BTreeMap<usize, efi::Font> = font_protocol.fonts();
    let graphics_output_protocol = efi::graphics_output::Protocol::get();
    let mp_services_protocol = efi::mp_services::Protocol::get();
    let my_processor_number: Option<usize> = mp_services_protocol.my_processor_number().ok();
    let processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation> = mp_services_protocol.get_all_processor_informations();
    let cpuid: x64::Cpuid = x64::Cpuid::get().unwrap();
    let execute_disable_bit_available: bool = x64::msr::Ia32Efer::enable_execute_disable_bit(&cpuid);
    assert!(execute_disable_bit_available);
    let mut paging = memory::Paging::get(&cpuid);
    paging.set();
    let directory_tree: efi::file::system::Tree = efi::file::system::Protocol::get().tree();
    let kernel: elf::File = directory_tree
        .get(KERNEL)
        .unwrap()
        .read()
        .into();
    let kernel_vaddr2frame: BTreeMap<usize, Box<memory::Frame>> = kernel.deploy(&mut paging);
    let kernel_stack_pages: usize = 0x10;
    let kernel_stack_vaddr2frame: BTreeMap<usize, Box<memory::Frame>> = (0..kernel_stack_pages)
        .map(|kernel_stack_page_index| (usize::MAX - (kernel_stack_page_index + 1) * memory::page::SIZE + 1, Box::default()))
        .collect();
    kernel_stack_vaddr2frame
        .iter()
        .for_each(|(vaddr, frame)| {
            let present: bool = true;
            let writable: bool = true;
            let executable: bool = false;
            paging.set_page(*vaddr, frame.paddr(), present, writable, executable);
        });
    let kernel_stack_floor: usize = 0;
    let memory_map: Vec<efi::memory::Descriptor> = efi::SystemTable::get()
        .memory_map()
        .unwrap()
        .into();
    let higher_half_range: Range<u128> = paging.higher_half_range();
    let kernel_heap_start: u128 = (higher_half_range.start + higher_half_range.end) / 2;
    let kernel_heap_start: usize = kernel_heap_start as usize;
    let kernel_heap_pages: usize = memory_map
        .into_iter()
        .filter(|memory_descriptor| memory_descriptor.is_available())
        .map(|memory_descriptor| memory_descriptor.number_of_pages())
        .sum();
    (0..kernel_heap_pages)
        .for_each(|heap_page_index| {
            let vaddr: usize = kernel_heap_start + heap_page_index * memory::page::SIZE;
            let paddr: usize = 0;
            let present: bool = false;
            let writable: bool = false;
            let executable: bool = false;
            paging.set_page(vaddr, paddr, present, writable, executable);
        });
    let processor_boot_loader: Vec<u8> = directory_tree
        .get(PROCESSOR_BOOT_LOADER)
        .unwrap()
        .read();
    let processor_boot_loader = processor::boot::Loader::new(&processor_boot_loader, processor_boot_loader_pages);
    let hello_application: elf::File = directory_tree
        .get("applications/hello.elf")
        .unwrap()
        .read()
        .into();
    let memory_map: efi::memory::Map = efi::SystemTable::get()
        .exit_boot_services(image_handle)
        .unwrap();
    let kernel_argument = kernel::Argument::new(
        processor_boot_loader,
        rs232c::get_com2(),
        cpuid,
        efi::SystemTable::get(),
        fonts,
        graphics_output_protocol,
        kernel_heap_start,
        hello_application,
        memory_map,
        my_processor_number,
        paging,
        processor_informations);
    kernel.run(kernel_stack_floor, &kernel_argument);
    efi::SystemTable::get().shutdown();
    efi::Status::ABORTED
}

/// # A panic handler of the boot loader
#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    com2_println!("BOOT PANIC!!!");
    com2_println!("{}", panic);
    loop {
        x64::hlt();
    }
}

