#![no_main]
#![no_std]
#![feature(abi_efiapi)]
#![allow(stable_features)]

use core::panic::PanicInfo;

mod asm;
mod rs232c;

#[no_mangle]
fn efi_main() {
    panic!("Hello, World!");
}

#[panic_handler]
fn panic(panic: &PanicInfo) -> ! {
    com2_println!("{}", panic);
    loop {
        asm::hlt();
    }
}

