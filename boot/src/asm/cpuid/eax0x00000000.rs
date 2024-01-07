//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    alloc::string::String,
    super::Return,
};

#[derive(Debug)]
pub struct Eax0x00000000 {
    max_eax: u32,
    vendor: String,
}

impl Eax0x00000000 {
    pub fn get() -> Option<Self> {
        let eax: u32 = 0x00000000;
        let ecx: u32 = 0x00000000;
        Return::get(eax, ecx).map(|cpuid_return| {
            let eax: u32 = cpuid_return.eax();
            let ebx: u32 = cpuid_return.ebx();
            let ecx: u32 = cpuid_return.ecx();
            let edx: u32 = cpuid_return.edx();
            let max_eax: u32 = eax;
            let vendor: String = [ebx, edx, ecx]
                .into_iter()
                .map(|dword| dword
                    .to_le_bytes()
                    .into_iter())
                .flatten()
                .filter_map(|byte| char::from_u32(byte as u32))
                .collect();
            Self {
                max_eax,
                vendor,
            }
        })
    }
}

