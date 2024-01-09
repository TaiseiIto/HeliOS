//! # Wrapper functions of x64 instructions
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)

use core::arch::asm;

pub mod control;
pub mod cpuid;
pub mod rflags;

pub use {
    cpuid::Cpuid,
    rflags::Rflags,
};

/// # Halt
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2A 3-489
#[inline(never)]
pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

/// # Input from port
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2A 3-505
#[inline(never)]
pub fn inb(port: u16) -> u8 {
    let mut data: u8;
    unsafe {
        asm!(
            "in al, dx",
            in("dx") port,
            out("al") data,
        );
    }
    data
}

/// # Output to port
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2B 4-176
#[inline(never)]
pub fn outb(port: u16, data: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") data,
        );
    }
}

/// # Read From Model Specific Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2B 4-4-537
#[allow(dead_code)]
#[inline(never)]
pub fn rdmsr(ecx: u32) -> u64 {
    let mut eax: u32;
    let mut edx: u32;
    unsafe {
        asm!(
            "rdmsr",
            in("ecx") ecx,
            out("eax") eax,
            out("edx") edx,
        );
    }
    (eax as u64) + ((edx as u64) << u32::BITS)
}

