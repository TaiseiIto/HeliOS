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
    super::Eax0x00000000,
};

#[derive(Debug)]
pub struct Eax0x0000000d {
    #[allow(dead_code)]
    ecx0x00000000: Ecx0x00000000,
    #[allow(dead_code)]
    ecx0x00000001: Ecx0x00000001,
    #[allow(dead_code)]
    ecx2ecxn: BTreeMap<u32, EcxN>,
}

impl Eax0x0000000d {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000000d;
        if eax <= eax0x00000000.max_eax() {
            let ecx0x00000000 = Ecx0x00000000::get(eax);
            let ecx0x00000001 = Ecx0x00000001::get(eax);
            let ecx2ecxn = (0..2 * u32::BITS)
                .filter(|n| ecx0x00000000.xcr0_n_is_valid(*n) || ecx0x00000001.ia32_xss_n_is_valid(*n))
                .map(|n| n + 2)
                .map(|ecx| (ecx, EcxN::get(eax, ecx)))
                .collect();
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

