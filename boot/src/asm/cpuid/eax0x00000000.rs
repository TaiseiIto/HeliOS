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
    #[allow(dead_code)]
    vendor: String,
}

impl Eax0x00000000 {
    pub fn get() -> Self {
        let eax: u32 = 0x00000000;
        let ecx: u32 = 0x00000000;
        let eax0x00000000 = Return::get(eax, ecx);
        let eax: u32 = eax0x00000000.eax();
        let ebx: u32 = eax0x00000000.ebx();
        let ecx: u32 = eax0x00000000.ecx();
        let edx: u32 = eax0x00000000.edx();
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
    }

    pub fn max_eax(&self) -> u32 {
        self.max_eax
    }
}

