//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::{
        Eax0x80000000,
        Return,
    },
};

#[derive(Debug)]
pub struct Eax0x80000006 {
    #[allow(dead_code)]
    ecx: Ecx,
}

impl Eax0x80000006 {
    pub fn get(eax0x80000000: &Eax0x80000000) -> Option<Self> {
        let eax: u32 = 0x80000006;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x80000000.max_eax()).then(|| {
            let eax0x80000006 = Return::get(eax, ecx);
            let ecx: Ecx = eax0x80000006.ecx().into();
            Self {
                ecx,
            }
        })
    }
}

#[bitfield(u32)]
struct Ecx {
    cache_line_size_in_bytes: u8,
    #[bits(4)]
    __: u8,
    #[bits(4)]
    l2_associativity_field: u8,
    cache_size_in_1k_units: u16,
}

