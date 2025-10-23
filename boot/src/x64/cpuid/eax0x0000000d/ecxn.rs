//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {super::super::Return, bitfield_struct::bitfield};

#[derive(Debug)]
pub struct EcxN {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl EcxN {
    pub fn get(eax: u32, ecx: u32) -> Self {
        let ecx0x00000000 = Return::get(eax, ecx);
        let eax: Eax = ecx0x00000000.eax().into();
        let ebx: Ebx = ecx0x00000000.ebx().into();
        let ecx: Ecx = ecx0x00000000.ecx().into();
        let edx: Edx = ecx0x00000000.edx().into();
        Self { eax, ebx, ecx, edx }
    }
}

#[bitfield(u32)]
struct Eax {
    size_in_bytes_of_the_save_area_for_an_extended_state_feature_associated_with_a_valid_sub_leaf_index_n:
        u32,
}

#[bitfield(u32)]
struct Ebx {
    the_offset_in_bytes_of_this_extended_state_components_save_area_from_the_beginning_of_the_xsave_xrstor_area:
        u32,
}

#[bitfield(u32)]
struct Ecx {
    bit_n_is_supported_in_the_ia32_xss_msr: bool,
    the_compacted_format_of_an_xsave_area_is_used: bool,
    #[bits(30)]
    __: u32,
}

#[bitfield(u32)]
struct Edx {
    valid: u32,
}
