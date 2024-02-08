//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::super::Return,
};

#[derive(Debug)]
pub struct Ecx0x00000000 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Ecx0x00000000 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000000;
        let ecx0x00000000 = Return::get(eax, ecx);
        let eax: Eax = ecx0x00000000.eax().into();
        let ebx: Ebx = ecx0x00000000.ebx().into();
        let ecx: Ecx = ecx0x00000000.ecx().into();
        let edx: Edx = ecx0x00000000.edx().into();
        Self {
            eax,
            ebx,
            ecx,
            edx,
        }
    }

    pub fn max_ecx(&self) -> u32 {
        self.eax.max_ecx()
    }
}

#[bitfield(u32)]
struct Eax {
    max_ecx: u32,
}

#[bitfield(u32)]
struct Ebx {
    page_size_4kb_entries_supported_by_this_structure: bool,
    page_size_2mb_entries_supported_by_this_structure: bool,
    page_size_4mb_entries_supported_by_this_structure: bool,
    page_size_1gb_entries_supported_by_this_structure: bool,
    #[bits(4, access = RO)]
    reserved0: u8,
    #[bits(3)]
    partitioning: u8,
    #[bits(5, access = RO)]
    reserved1: u8,
    ways_of_associativity: u16,
}

#[bitfield(u32)]
struct Ecx {
    number_of_sets: u32,
}

#[bitfield(u32)]
struct Edx {
    #[bits(5, access = RO)]
    translation_cache_type_field: u8,
    #[bits(3)]
    translation_cache_level: u8,
    fully_associative_structure: bool,
    #[bits(5, access = RO)]
    reserved0: u8,
    #[bits(12)]
    maximum_number_of_addressable_ids_for_logical_processors_sharing_this_translation_cache: u16,
    #[bits(6, access = RO)]
    reserved1: u8,
}

