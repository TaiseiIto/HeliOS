//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    super::{Eax0x00000000, Return},
    bitfield_struct::bitfield,
};

#[derive(Debug)]
pub struct Eax0x00000015 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
}

impl Eax0x00000015 {
    pub fn frequency(&self) -> Option<u64> {
        let denominator: u64 = self.eax.denominator() as u64;
        let numerator: u64 = self.ebx.numerator() as u64;
        let frequency: u64 = self.ecx.frequency() as u64;
        (frequency != 0 && denominator != 0).then(|| frequency * numerator / denominator)
    }

    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000015;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let eax0x00000015 = Return::get(eax, ecx);
            let eax: Eax = eax0x00000015.eax().into();
            let ebx: Ebx = eax0x00000015.ebx().into();
            let ecx: Ecx = eax0x00000015.ecx().into();
            Self { eax, ebx, ecx }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    denominator: u32,
}

#[bitfield(u32)]
struct Ebx {
    numerator: u32,
}

#[bitfield(u32)]
struct Ecx {
    frequency: u32,
}
