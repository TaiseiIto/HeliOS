//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    super::{Eax0x00000000, Eax0x00000007, Return},
    alloc::collections::BTreeSet,
};

#[derive(Debug)]
pub struct Eax0x0000001b {
    #[allow(dead_code)]
    target_identifiers: BTreeSet<u32>,
}

impl Eax0x0000001b {
    pub fn get(
        eax0x00000000: &Eax0x00000000,
        eax0x00000007: &Option<Eax0x00000007>,
    ) -> Option<Self> {
        let eax: u32 = 0x0000001b;
        eax0x00000007.as_ref().and_then(|eax0x00000007| {
            (eax <= eax0x00000000.max_eax() && eax0x00000007.pconfig()).then(|| {
                let target_identifiers: BTreeSet<u32> = (0..)
                    .map(|ecx| Return::get(eax, ecx))
                    .take_while(|cpuid_return| cpuid_return.eax() != 0)
                    .flat_map(|cpuid_return| {
                        let ebx: u32 = cpuid_return.ebx();
                        let ecx: u32 = cpuid_return.ecx();
                        let edx: u32 = cpuid_return.edx();
                        [ebx, ecx, edx]
                            .into_iter()
                            .filter(|target_identifier| *target_identifier != 0)
                    })
                    .collect();
                Self { target_identifiers }
            })
        })
    }
}
