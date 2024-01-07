//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use super::Return;

#[derive(Debug)]
pub struct Eax0x00000002 {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

