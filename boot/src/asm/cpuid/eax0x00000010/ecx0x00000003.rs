//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::super::Return,
};

#[derive(Debug)]
pub struct Ecx0x00000003 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Ecx0x00000003 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000001;
        let ecx0x00000003 = Return::get(eax, ecx);
        let eax: Eax = ecx0x00000003.eax().into();
        let ecx: Ecx = ecx0x00000003.ecx().into();
        let edx: Edx = ecx0x00000003.edx().into();
        Self {
            eax,
            ecx,
            edx,
        }
    }
}

#[bitfield(u32)]
struct Eax {
    #[bits(12)]
    the_maximum_mba_throttling_value_supported_for_the_corresponding_resid: u16,
    #[bits(20, access = RO)]
    reserved0: u32,
}

#[bitfield(u32)]
struct Ecx {
    #[bits(2, access = RO)]
    reserved0: u8,
    the_responce_of_the_delay_values_is_linear: bool,
    #[bits(29, access = RO)]
    reserved1: u32,
}

#[bitfield(u32)]
struct Edx {
    highest_cos_number_supported_for_this_resid: u16,
    reserved0: u16,
}

