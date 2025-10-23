//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {super::Return, bitfield_struct::bitfield};

#[derive(Debug)]
pub struct Eax0x80000000 {
    eax: Eax,
}

impl Eax0x80000000 {
    pub fn get() -> Self {
        let eax: u32 = 0x80000000;
        let ecx: u32 = 0x00000000;
        let eax0x80000000 = Return::get(eax, ecx);
        let eax: Eax = eax0x80000000.eax().into();
        Self { eax }
    }

    pub fn max_eax(&self) -> u32 {
        self.eax.max_eax()
    }
}

#[bitfield(u32)]
struct Eax {
    max_eax: u32,
}
