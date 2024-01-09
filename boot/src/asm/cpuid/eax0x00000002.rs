//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    alloc::collections::BTreeSet,
    super::{
        Eax0x00000000,
        Return,
    },
};

#[derive(Debug)]
pub struct Eax0x00000002 {
    #[allow(dead_code)]
    cache_and_tlb_information: BTreeSet<u8>,
}

impl Eax0x00000002 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000002;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let eax0x00000002 = Return::get(eax, ecx);
            let eax: u32 = eax0x00000002.eax() & 0xffffff00;
            let ebx: u32 = eax0x00000002.ebx();
            let ecx: u32 = eax0x00000002.ecx();
            let edx: u32 = eax0x00000002.edx();
            let cache_and_tlb_information: BTreeSet<u8> = [eax, ebx, ecx, edx]
                .into_iter()
                .filter(|dword| dword & 0x80000000 == 0)
                .flat_map(|dword| dword
                    .to_le_bytes()
                    .into_iter()
                    .filter(|byte| *byte != 0))
                .collect();
            Some(Self {
                cache_and_tlb_information,
            })
        } else {
            None
        }
    }
}

