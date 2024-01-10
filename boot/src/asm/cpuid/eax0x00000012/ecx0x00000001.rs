//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use super::super::Return;

#[derive(Debug)]
pub struct Ecx0x00000001 {
    #[allow(dead_code)]
    the_valid_bits_of_secs_attributes_that_software_can_set_with_ecreate: u128,
}

impl Ecx0x00000001 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000001;
        let ecx0x00000001 = Return::get(eax, ecx);
        let eax: u32 = ecx0x00000001.eax();
        let ebx: u32 = ecx0x00000001.ebx();
        let ecx: u32 = ecx0x00000001.ecx();
        let edx: u32 = ecx0x00000001.edx();
        let the_valid_bits_of_secs_attributes_that_software_can_set_with_ecreate: u128 = [eax, ebx, ecx, edx]
            .into_iter()
            .flat_map(|dword| dword
                .to_le_bytes()
                .into_iter())
            .rev()
            .fold(0, |the_valid_bits_of_secs_attributes_that_software_can_set_with_ecreate, byte| (the_valid_bits_of_secs_attributes_that_software_can_set_with_ecreate << u8::BITS) + (byte as u128));
        Self {
            the_valid_bits_of_secs_attributes_that_software_can_set_with_ecreate,
        }
    }
}

