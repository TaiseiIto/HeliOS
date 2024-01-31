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
pub struct Eax0x80000007 {
    #[allow(dead_code)]
    edx: Edx,
}

impl Eax0x80000007 {
    pub fn get(eax0x80000000: &Eax0x80000000) -> Option<Self> {
        let eax: u32 = 0x80000007;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x80000000.max_eax()).then(|| {
            let eax0x80000007 = Return::get(eax, ecx);
            let edx: Edx = eax0x80000007.edx().into();
            Self {
                edx,
            }
        })
    }
}

#[bitfield(u32)]
struct Edx {
    #[bits(access = RO)]
    reserved0: u8,
    invariant_tsc_available: bool,
    #[bits(23, access = RO)]
    reserved1: u32,
}

