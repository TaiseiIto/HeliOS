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
pub struct Eax0x0000001e {
    #[allow(dead_code)]
    ebx: Ebx,
}

impl Eax0x0000001e {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000001e;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let eax0x0000001e = Return::get(eax, ecx);
            let ebx: Ebx = eax0x0000001e.ebx().into();
            Some(Self {
                ebx,
            })
        } else {
            None
        }
    }
}

#[bitfield(u32)]
struct Ebx {
    tmul_maxk: u8,
    tmul_maxn: u16,
    #[bits(access = RO)]
    reserved0: u8,
}

