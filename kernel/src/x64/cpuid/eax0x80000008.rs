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
pub struct Eax0x80000008 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
}

impl Eax0x80000008 {
    pub fn get(eax0x80000000: &Eax0x80000000) -> Option<Self> {
        let eax: u32 = 0x80000008;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x80000000.max_eax()).then(|| {
            let eax0x80000008 = Return::get(eax, ecx);
            let eax: Eax = eax0x80000008.eax().into();
            let ebx: Ebx = eax0x80000008.ebx().into();
            Self {
                eax,
                ebx,
            }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    physical_address_bits: u8,
    linear_address_bits: u8,
    __: u16,
}

#[bitfield(u32)]
struct Ebx {
    #[bits(9)]
    __: u16,
    wbnoinvd_is_available: bool,
    #[bits(22)]
    __: u32,
}

