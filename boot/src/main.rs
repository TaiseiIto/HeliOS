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
    let processor_boot_loader_pages: Range<efi::memory::PhysicalAddress> = processor::boot::Loader::allocate_pages(PROCESSOR_BOOT_LOADER_BASE, PROCESSOR_BOOT_LOADER_STACK_FLOOR);
    efi_println!("Hello, World!");
    com2_println!("Hello from /EFI/BOOT/BOOTX64.EFI");
    let font_protocol = efi::font::Protocol::get();
    let fonts: BTreeMap<usize, efi::Font> = font_protocol.fonts();
    let graphics_output_protocol = efi::graphics_output::Protocol::get();
    let cpuid: x64::Cpuid = x64::Cpuid::get().unwrap();
    let execute_disable_bit_available: bool = x64::msr::Ia32Efer::enable_execute_disable_bit(&cpuid);
    assert!(execute_disable_bit_available);
    let mut paging = memory::Paging::get(&cpuid);
    paging.set();
    let directory_tree: efi::file::system::Tree = efi::file::system::Protocol::get().tree();
    let kernel = kernel::Loader::new(KERNEL, &directory_tree, &mut paging);
    let kernel_heap_start: usize = kernel.heap_start();
    let processor_boot_loader: Vec<u8> = directory_tree
        .get(PROCESSOR_BOOT_LOADER)
        .unwrap()
        .read();
    let processor_boot_loader = processor::boot::Loader::new(&processor_boot_loader, processor_boot_loader_pages);
    let processor_kernel: Vec<u8> = directory_tree
        .get(PROCESSOR_KERNEL)
        .unwrap()
        .read();
    let memory_map: efi::memory::Map = efi::SystemTable::get()
        .exit_boot_services(image_handle)
        .unwrap();
    let kernel_argument = kernel::Argument::new(
        processor_boot_loader,
        processor_kernel,
        rs232c::get_com2(),
        cpuid,
        efi::SystemTable::get(),
        fonts,
        graphics_output_protocol,
        kernel_heap_start,
        memory_map,
        paging);
    kernel.run(&kernel_argument);
    unreachable!("Failure to start the kernel.")
}

/// # A panic handler of the boot loader
#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    com2_println!("BOOT PANIC!!!");
    com2_println!("{}", panic);
    efi::SystemTable::get().shutdown();
    loop {
        x64::hlt();
    }
}

