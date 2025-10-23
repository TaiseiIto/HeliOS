//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

mod ecx0x00000000;
mod ecxn;

use {
    super::Eax0x00000000, alloc::collections::BTreeMap, ecx0x00000000::Ecx0x00000000, ecxn::EcxN,
};

#[derive(Debug)]
pub struct Eax0x00000018 {
    #[allow(dead_code)]
    ecx0x00000000: Ecx0x00000000,
    #[allow(dead_code)]
    ecxn: BTreeMap<u32, EcxN>,
}

impl Eax0x00000018 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000018;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let ecx0x00000000 = Ecx0x00000000::get(eax);
            let ecxn: BTreeMap<u32, EcxN> = (1..=ecx0x00000000.max_ecx())
                .filter_map(|ecx| EcxN::get(eax, ecx).map(|ecxn| (ecx, ecxn)))
                .collect();
            Self {
                ecx0x00000000,
                ecxn,
            }
        })
    }
}
