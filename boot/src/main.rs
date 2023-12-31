//! # The bootloader

#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

extern crate alloc;

use core::panic::PanicInfo;

mod asm;
mod efi;
mod rs232c;

/// # The entry point of the OS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.1 UEFI Image Entry Point
#[no_mangle]
fn efi_main(image_handle: efi::Handle, system_table: &'static mut efi::SystemTable<'static>) -> efi::Status {
    system_table.set();
    com2_println!("image_handle = {:#x?}", image_handle);
    com2_println!("system_table = {:#x?}", efi::SystemTable::get());
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

