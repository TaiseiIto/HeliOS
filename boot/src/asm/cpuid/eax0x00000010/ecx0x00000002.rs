//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::super::Return,
};

#[derive(Debug)]
pub struct Ecx0x00000002 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Ecx0x00000002 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000001;
        let ecx0x00000002 = Return::get(eax, ecx);
        let eax: Eax = ecx0x00000002.eax().into();
        let ebx: Ebx = ecx0x00000002.ebx().into();
        let ecx: Ecx = ecx0x00000002.ecx().into();
        let edx: Edx = ecx0x00000002.edx().into();
        Self {
            eax,
            ebx,
            ecx,
            edx,
        }
    }
}

#[bitfield(u32)]
struct Eax {
    #[bits(5)]
    length_of_the_capacity_bit_mask_for_the_corresponding_resid: u8,
    #[bits(27, access = RO)]
    reserved0: u32,
}

#[bitfield(u32)]
struct Ebx {
    bit_granular_map_of_isolation_contention_of_allocation_units: u32,
}

#[bitfield(u32)]
struct Ecx {
    #[bits(2, access = RO)]
    reserved0: u8,
    l2_code_and_data_prioritization_technology_is_supported: bool,
    non_contiguous_capacity_bitmask_is_supported: bool,
    #[bits(28, access = RO)]
    reserved1: u32,
}

#[bitfield(u32)]
struct Edx {
    highest_cos_number_supported_for_this_resid: u16,
    #[bits(access = RO)]
    reserved0: u16,
}

