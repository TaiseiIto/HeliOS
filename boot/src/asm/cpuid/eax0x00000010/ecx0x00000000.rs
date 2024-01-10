//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::super::Return,
};

#[derive(Debug)]
pub struct Ecx0x00000000 {
    #[allow(dead_code)]
    ebx: Ebx,
}

impl Ecx0x00000000 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000000;
        let ecx0x00000000 = Return::get(eax, ecx);
        let ebx: Ebx = ecx0x00000000.ebx().into();
        Self {
            ebx,
        }
    }
}

#[bitfield(u32)]
struct Ebx {
    reserved0: bool,
    supports_l3_cache_allocation_technology: bool,
    supports_l2_cache_allocation_technology: bool,
    supports_memory_bandwidth_allocation: bool,
    #[bits(28, access = RO)]
    reserved1: u32,
}

