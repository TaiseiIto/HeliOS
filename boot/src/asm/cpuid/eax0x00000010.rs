//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

mod ecx0x00000000;

use super::Eax0x00000000;

pub use {
    ecx0x00000000::Ecx0x00000000,
};

#[derive(Debug)]
pub struct Eax0x00000010 {
    #[allow(dead_code)]
    ecx0x00000000: Ecx0x00000000,
}

impl Eax0x00000010 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000010;
        if eax <= eax0x00000000.max_eax() {
            let ecx0x00000000 = Ecx0x00000000::get(eax);
            Some(Self {
                ecx0x00000000,
            })
        } else {
            None
        }
    }
}

