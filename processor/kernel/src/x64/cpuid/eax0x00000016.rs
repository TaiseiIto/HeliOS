//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    super::{Eax0x00000000, Return},
    bitfield_struct::bitfield,
};

#[derive(Debug)]
pub struct Eax0x00000016 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
}

impl Eax0x00000016 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000016;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let eax0x00000016 = Return::get(eax, ecx);
            let eax: Eax = eax0x00000016.eax().into();
            let ebx: Ebx = eax0x00000016.ebx().into();
            let ecx: Ecx = eax0x00000016.ecx().into();
            Self { eax, ebx, ecx }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    processor_base_frequency_in_mhz: u16,
    __: u16,
}

#[bitfield(u32)]
struct Ebx {
    maximum_frequency_in_mhz: u16,
    __: u16,
}

#[bitfield(u32)]
struct Ecx {
    bus_reference_frequency_in_mhz: u16,
    __: u16,
}
