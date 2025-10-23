//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

mod ecx0x00000000;
mod ecx0x00000001;
mod ecx0x00000002;
mod ecx0x00000003;

use {
    super::Eax0x00000000, ecx0x00000000::Ecx0x00000000, ecx0x00000001::Ecx0x00000001,
    ecx0x00000002::Ecx0x00000002, ecx0x00000003::Ecx0x00000003,
};

#[derive(Debug)]
pub struct Eax0x00000010 {
    #[allow(dead_code)]
    ecx0x00000000: Ecx0x00000000,
    #[allow(dead_code)]
    ecx0x00000001: Ecx0x00000001,
    #[allow(dead_code)]
    ecx0x00000002: Ecx0x00000002,
    #[allow(dead_code)]
    ecx0x00000003: Ecx0x00000003,
}

impl Eax0x00000010 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000010;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let ecx0x00000000 = Ecx0x00000000::get(eax);
            let ecx0x00000001 = Ecx0x00000001::get(eax);
            let ecx0x00000002 = Ecx0x00000002::get(eax);
            let ecx0x00000003 = Ecx0x00000003::get(eax);
            Self {
                ecx0x00000000,
                ecx0x00000001,
                ecx0x00000002,
                ecx0x00000003,
            }
        })
    }
}
