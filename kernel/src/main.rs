//! The kernel

#![no_main]
#![no_std]

use core::{
    arch::asm,
    panic::PanicInfo,
};

#[no_mangle]
fn main() {
    panic!("Kernel Panic!");
}

/// # A panic handler of the kernel
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "out dx, al",
            in("al") 0x48i8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x65i8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x6ci8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x6ci8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x6fi8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x2ci8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x20i8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x6bi8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x65i8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x72i8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x6ei8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x65i8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x6ci8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x2ei8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x65i8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x6ci8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x66i8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x21i8,
            in("dx") 0x02f8i16,
        );
        asm!(
            "out dx, al",
            in("al") 0x0ai8,
            in("dx") 0x02f8i16,
        );
    }
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

