//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use super::Return;

#[derive(Debug)]
pub struct Eax0x00000000 {
    max_eax: u32,
}

impl Eax0x00000000 {
    pub fn get() -> Self {
        let eax: u32 = 0x00000000;
        let ecx: u32 = 0x00000000;
        let eax0x00000000 = Return::get(eax, ecx);
        let eax: u32 = eax0x00000000.eax();
        let max_eax: u32 = eax;
        Self {
            max_eax,
        }
    }

    pub fn max_eax(&self) -> u32 {
        self.max_eax
    }
}

