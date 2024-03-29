//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::{
        Eax0x00000000,
        Return,
    },
};

#[derive(Debug)]
pub struct Eax0x00000009 {
    #[allow(dead_code)]
    eax: Eax,
}

impl Eax0x00000009 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000009;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let eax0x00000009 = Return::get(eax, ecx);
            let eax: Eax = eax0x00000009.eax().into();
            Self {
                eax,
            }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    ia32_platform_dca_cap_msr: u32,
}

