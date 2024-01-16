//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

mod ecx0x00000000;

use {
    alloc::string::String,
    super::{
        Eax0x00000000,
        Return,
    },
    ecx0x00000000::Ecx0x00000000,
};

#[derive(Debug)]
pub struct Eax0x00000017 {
    #[allow(dead_code)]
    ecx0x00000000: Ecx0x00000000,
    #[allow(dead_code)]
    soc_vendor_brand_string: Option<String>,
}

impl Eax0x00000017 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000017;
        if eax <= eax0x00000000.max_eax() {
            let ecx0x00000000 = Ecx0x00000000::get(eax);
            let soc_vendor_brand_string = String::from_utf8((1..=3)
                .flat_map(|ecx| Return::get(eax, ecx)
                    .eax_ebx_ecx_edx()
                    .into_iter()
                    .flat_map(|dword| dword
                        .to_le_bytes()
                        .into_iter()))
                .collect()).ok();
            Some(Self {
                ecx0x00000000,
                soc_vendor_brand_string,
            })
        } else {
            None
        }
    }
}

