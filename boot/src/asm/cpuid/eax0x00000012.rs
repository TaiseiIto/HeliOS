//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

mod ecx0x00000000;
mod ecx0x00000001;
mod ecxn;

use {
    alloc::collections::BTreeMap,
    ecx0x00000000::Ecx0x00000000,
    ecx0x00000001::Ecx0x00000001,
    ecxn::EcxN,
    super::{
        Eax0x00000000,
        Eax0x00000007,
    },
};

#[derive(Debug)]
pub struct Eax0x00000012 {
    #[allow(dead_code)]
    ecx0x00000000: Ecx0x00000000,
    #[allow(dead_code)]
    ecx0x00000001: Ecx0x00000001,
    #[allow(dead_code)]
    ecx2ecxn: BTreeMap<u32, EcxN>,
}

impl Eax0x00000012 {
    pub fn get(eax0x00000000: &Eax0x00000000, eax0x00000007: &Option<Eax0x00000007>) -> Option<Self> {
        let eax: u32 = 0x00000012;
        if eax <= eax0x00000000.max_eax() {
            let ecx0x00000000 = Ecx0x00000000::get(eax);
            let ecx0x00000001 = Ecx0x00000001::get(eax);
            let ecx2ecxn: BTreeMap<u32, EcxN> = if eax0x00000007.as_ref().map_or(false, |eax0x00000007| eax0x00000007.sgx()) {
                (2..)
                    .map(|ecx| (ecx, EcxN::get(eax, ecx)))
                    .take_while(|ecx_and_ecxn| ecx_and_ecxn.1.is_some())
                    .filter_map(|ecx_and_ecxn| ecx_and_ecxn.1.map(|ecxn| (ecx_and_ecxn.0, ecxn)))
                    .collect()
            } else {
                BTreeMap::<u32, EcxN>::new()
            };
            Some(Self {
                ecx0x00000000,
                ecx0x00000001,
                ecx2ecxn,
            })
        } else {
            None
        }
    }
}


