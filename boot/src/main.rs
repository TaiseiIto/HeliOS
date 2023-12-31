#![no_main]
#![no_std]
#![feature(abi_efiabi)]
#![allow(stable_features)]

use core::arch::asm;

mod asm;

#![no_mangle]
fn efi_main() {
    panic!("Hello, World!");
}

#![panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    loop {
        asm::hlt();
    }
}

