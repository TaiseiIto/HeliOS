//! # The bootloader

#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

extern crate alloc;

use {
    alloc::{
        collections::BTreeMap,
        vec::Vec,
    },
    core::panic::PanicInfo,
};

mod efi;
mod interrupt;
mod memory;
mod rs232c;
mod x64;

/// # The entry point of the OS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.1 UEFI Image Entry Point
#[no_mangle]
fn efi_main(image_handle: efi::Handle, system_table: &'static mut efi::SystemTable<'static>) -> efi::Status {
    system_table.set();
    com2_println!("image_handle = {:#x?}", image_handle);
    com2_println!("system_table = {:#x?}", efi::SystemTable::get());
    let mp_services_protocol = efi::mp_services::Protocol::get();
    com2_println!("mp_services_protocol = {:#x?}", mp_services_protocol);
    let my_processor_number = mp_services_protocol
        .my_processor_number()
        .unwrap();
    com2_println!("my_processor_number = {:#x?}", my_processor_number);
    let processor_informations: BTreeMap<usize, efi::mp_services::ProcessorInformation> = mp_services_protocol.get_all_processor_informations();
    com2_println!("processor_informations = {:#x?}", processor_informations);
    let cpuid = x64::Cpuid::get();
    com2_println!("cpuid = {:#x?}", cpuid);
    let _paging = memory::Paging::get(&cpuid);
    let gdt = memory::segment::descriptor::Table::get();
    com2_println!("gdt = {:#x?}", gdt);
    let idt = interrupt::descriptor::Table::get();
    com2_println!("idt = {:#x?}", idt);
    let memory_map: Vec<efi::memory::Descriptor> = efi::SystemTable::get()
        .memory_map()
        .unwrap()
        .into();
    com2_println!("memory_map = {:#x?}", memory_map);
    efi_println!("Hello, World!");
    let memory_map: efi::memory::Map = efi::SystemTable::get()
        .exit_boot_services(image_handle)
        .unwrap();
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

