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
pub struct Eax0x0000001a {
    #[allow(dead_code)]
    eax: Eax,
}

impl Eax0x0000001a {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000001a;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let eax0x0000001a = Return::get(eax, ecx);
            let eax: Eax = eax0x0000001a.eax().into();
            Some(Self {
                eax,
            })
        } else {
            None
        }
    }
}

#[bitfield(u32)]
struct Eax {
    #[bits(24)]
    native_model_id_of_the_core: u32,
    core_type: u8,
}

