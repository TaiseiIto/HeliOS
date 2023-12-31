#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

use core::panic::PanicInfo;

mod asm;

#[no_mangle]
fn efi_main() {
    panic!("Hello, World!");
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {
        asm::hlt();
    }
}

