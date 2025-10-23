//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

mod ecx0x00000000;
mod ecx0x00000001;
mod ecxn;

use {
    super::{Eax0x00000000, Eax0x00000007},
    alloc::collections::BTreeMap,
    ecx0x00000000::Ecx0x00000000,
    ecx0x00000001::Ecx0x00000001,
    ecxn::EcxN,
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
    pub fn get(
        eax0x00000000: &Eax0x00000000,
        eax0x00000007: &Option<Eax0x00000007>,
    ) -> Option<Self> {
        let eax: u32 = 0x00000012;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let ecx0x00000000 = Ecx0x00000000::get(eax);
            let ecx0x00000001 = Ecx0x00000001::get(eax);
            let ecx2ecxn: BTreeMap<u32, EcxN> = eax0x00000007
                .as_ref()
                .map_or(false, |eax0x00000007| eax0x00000007.sgx())
                .then(|| {
                    (2..)
                        .map(|ecx| EcxN::get(eax, ecx).map(|ecxn| (ecx, ecxn)))
                        .take_while(|ecx_and_ecxn| ecx_and_ecxn.is_some())
                        .flatten()
                        .collect()
                })
                .unwrap_or_default();
            Self {
                ecx0x00000000,
                ecx0x00000001,
                ecx2ecxn,
            }
        })
    }
}
