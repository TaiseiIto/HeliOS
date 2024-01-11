//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use super::{
    Eax0x00000000,
    Eax0x00000007,
};

#[derive(Debug)]
pub struct Eax0x0000001b {
}

impl Eax0x0000001b {
    pub fn get(eax0x00000000: &Eax0x00000000, eax0x00000007: &Option<Eax0x00000007>) -> Option<Self> {
        let eax: u32 = 0x0000001b;
        eax0x00000007
            .as_ref()
            .and_then(|eax0x00000007| if eax <= eax0x00000000.max_eax() && eax0x00000007.pconfig() {
                Some(Self {
                })
            } else {
                None
            })
    }
}

