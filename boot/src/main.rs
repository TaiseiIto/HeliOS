#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

use core::panic::PanicInfo;

mod asm;
mod rs232c;

#[no_mangle]
fn efi_main() {
    let com2 = rs232c::com2();
    panic!("Hello, World!");
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {
        asm::hlt();
    }
}

