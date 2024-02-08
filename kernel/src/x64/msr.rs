//! # Model Specific Registers
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4

mod ia32_efer;

use core::arch::asm;

pub use ia32_efer::Ia32Efer;

/// # Read From Model Specific Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2B 4-4-537
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

/// # Read From Model Specific Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.2D 6-9
#[inline(never)]
pub fn wrmsr(ecx: u32, value: u64) {
    let eax: u32 = (value & 0x00000000ffffffff) as u32;
    let edx: u32 = (value >> u32::BITS) as u32;
    unsafe {
        asm!(
            "wrmsr",
            in("eax") eax,
            in("ecx") ecx,
            in("edx") edx,
        );
    }
}

