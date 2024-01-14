//! # Wrapper functions of x64 instructions
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)

use core::arch::asm;

pub mod control;
pub mod cpuid;
pub mod msr;
pub mod port;
pub mod rflags;
pub mod segment;

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

