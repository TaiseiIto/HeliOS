//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {super::super::Return, bitfield_struct::bitfield};

#[derive(Debug)]
pub struct EcxN {
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl EcxN {
    pub fn get(eax: u32, ecx: u32) -> Option<Self> {
        let ecxn = Return::get(eax, ecx);
        let ebx: Ebx = ecxn.ebx().into();
        let ecx: Ecx = ecxn.ecx().into();
        let edx: Edx = ecxn.edx().into();
        match edx.translation_cache_type_field() {
            0 => None,
            _ => Some(Self { ebx, ecx, edx }),
        }
    }
}

#[bitfield(u32)]
struct Ebx {
    page_size_4k_entries_supported_by_this_structure: bool,
    page_size_2m_entries_supported_by_this_structure: bool,
    page_size_4m_entries_supported_by_this_structure: bool,
    page_size_1g_entries_supported_by_this_structure: bool,
    #[bits(4)]
    __: u8,
    #[bits(3)]
    partitioning: u8,
    #[bits(5)]
    __: u8,
    ways_of_associativity: u16,
}

#[bitfield(u32)]
struct Ecx {
    number_of_sets: u32,
}

#[bitfield(u32)]
struct Edx {
    #[bits(5)]
    translation_cache_type_field: u8,
    #[bits(3)]
    translation_cache_level: u8,
    fully_associative_structure: bool,
    #[bits(5)]
    __: u8,
    #[bits(12)]
    maximum_number_of_addressable_ids_for_logical_processors_sharing_this_translation_cache: u16,
    #[bits(6)]
    __: u8,
}
