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
pub struct Eax0x00000010 {
    #[allow(dead_code)]
    ebx: Ebx,
}

impl Eax0x00000010 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000010;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let eax0x0000000f = Return::get(eax, ecx);
            let ebx: Ebx = eax0x0000000f.ebx().into();
            Some(Self {
                ebx,
            })
        } else {
            None
        }
    }
}

#[bitfield(u32)]
pub struct Ebx {
    reserved0: bool,
    supports_l3_cache_allocation_technology: bool,
    supports_l2_cache_allocation_technology: bool,
    supports_memory_bandwidth_allocation: bool,
    #[bits(28, access = RO)]
    reserved1: u32,
}

