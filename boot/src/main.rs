//! # The bootloader

#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

extern crate alloc;

use {
    alloc::vec::Vec,
    core::panic::PanicInfo,
};

mod asm;
mod efi;
mod memory;
mod rs232c;

/// # The entry point of the OS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.1 UEFI Image Entry Point
#[no_mangle]
fn efi_main(image_handle: efi::Handle, system_table: &'static mut efi::SystemTable<'static>) -> efi::Status {
    system_table.set();
    com2_println!("image_handle = {:#x?}", image_handle);
    com2_println!("system_table = {:#x?}", efi::SystemTable::get());
    let memory_map: Vec<efi::memory::Descriptor> = efi::SystemTable::get().memory_map();
    com2_println!("memory_map = {:#x?}", memory_map);
    let cr0 = asm::control::Register0::get();
    com2_println!("cr0 = {:#x?}", cr0);
    let cr2 = asm::control::Register2::get();
    com2_println!("cr2 = {:#x?}", cr2);
    let cr3 = asm::control::Register3::get();
    com2_println!("cr3 = {:#x?}", cr3);
    let cr4 = asm::control::Register4::get();
    com2_println!("cr4 = {:#x?}", cr4);
    let cpuid = asm::Cpuid::get();
    com2_println!("cpuid = {:#x?}", cpuid);
    let ia32_efer = asm::msr::Ia32Efer::get(&cpuid);
    com2_println!("ia32_efer = {:#x?}", ia32_efer);
    let paging = memory::Paging::get();
    com2_println!("paging = {:#x?}", paging);
    efi_println!("Hello, World!");
    efi::SystemTable::get().shutdown();
    efi::Status::ABORTED
}

/// # A panic handler of the boot loader
#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    com2_println!("BOOT PANIC!!!");
    com2_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

