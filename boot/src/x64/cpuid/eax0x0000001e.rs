//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    super::{Eax0x00000000, Return},
    bitfield_struct::bitfield,
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
        (eax <= eax0x00000000.max_eax()).then(|| {
            let eax0x0000001e = Return::get(eax, ecx);
            let ebx: Ebx = eax0x0000001e.ebx().into();
            Self { ebx }
        })
    }
}

#[bitfield(u32)]
struct Ebx {
    tmul_maxk: u8,
    tmul_maxn: u16,
    __: u8,
}
