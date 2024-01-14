//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use super::{
    Eax0x00000000,
    Eax0x00000001,
    Return,
};

#[derive(Debug)]
pub struct Eax0x00000003 {
    #[allow(dead_code)]
    processor_serial_number: u64,
}

impl Eax0x00000003 {
    pub fn get(eax0x00000000: &Eax0x00000000, eax0x00000001: &Option<Eax0x00000001>) -> Option<Self> {
        let eax: u32 = 0x00000003;
        let ecx: u32 = 0x00000000;
        eax0x00000001
            .as_ref()
            .and_then(|eax0x00000001| if eax <= eax0x00000000.max_eax() && eax0x00000001.psn() {
                let eax0x00000003 = Return::get(eax, ecx);
                let ecx: u32 = eax0x00000003.ecx();
                let edx: u32 = eax0x00000003.edx();
                let processor_serial_number: u64 = (ecx as u64) | ((edx as u64) << u32::BITS);
                Some(Self {
                    processor_serial_number
                })
            } else {
                None
            })
    }
}

