//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    super::{super::Return, Ecx0x00000000},
    bitfield_struct::bitfield,
};

#[derive(Debug)]
pub struct Ecx0x00000001 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
}

impl Ecx0x00000001 {
    pub fn get(eax: u32, ecx0x00000000: &Ecx0x00000000) -> Option<Self> {
        let ecx: u32 = 0x00000001;
        (ecx <= ecx0x00000000.max_ecx()).then(|| {
            let ecx0x00000000 = Return::get(eax, ecx);
            let eax: Eax = ecx0x00000000.eax().into();
            let ebx: Ebx = ecx0x00000000.ebx().into();
            Self { eax, ebx }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    #[bits(3)]
    number_of_configurable_address_ranges_for_filtering: u8,
    #[bits(13)]
    __: u16,
    bitmap_of_supportem_mtc_period_encoding: u16,
}

#[bitfield(u32)]
struct Ebx {
    bitmap_of_supported_cycle_threshold_value_encodings: u16,
    bitmap_of_supported_configurable_psb_frequency_encodings: u16,
}
