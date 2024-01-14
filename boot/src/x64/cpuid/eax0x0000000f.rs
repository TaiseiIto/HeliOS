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
pub struct Eax0x0000000f {
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Eax0x0000000f {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000000f;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let eax0x0000000f = Return::get(eax, ecx);
            let ebx: Ebx = eax0x0000000f.ebx().into();
            let edx: Edx = eax0x0000000f.edx().into();
            Some(Self {
                ebx,
                edx,
            })
        } else {
            None
        }
    }
}

#[bitfield(u32)]
struct Ebx {
    maximum_range_of_rmid_within_this_physical_processor_of_all_types: u32,
}

#[bitfield(u32)]
struct Edx {
    #[bits(access = RO)]
    reserved0: bool,
    supports_l3_cache_intel_rdt_monitoring: bool,
    #[bits(30, access = RO)]
    reserved1: u32,
}

