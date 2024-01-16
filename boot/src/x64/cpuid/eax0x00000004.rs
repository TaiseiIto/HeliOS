//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::{
        Eax0x00000000,
        Return,
    },
};

#[derive(Debug)]
pub struct Eax0x00000004 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Eax0x00000004 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000004;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let eax0x00000004 = Return::get(eax, ecx);
            let eax: Eax = eax0x00000004.eax().into();
            let ebx: Ebx = eax0x00000004.ebx().into();
            let ecx: Ecx = eax0x00000004.ecx().into();
            let edx: Edx = eax0x00000004.edx().into();
            Some(Self {
                eax,
                ebx,
                ecx,
                edx,
            })
        } else {
            None
        }
    }
}

#[bitfield(u32)]
struct Eax {
    #[bits(5)]
    cache_type_field: u8,
    #[bits(3)]
    cache_level: u8,
    self_initializing_cache_level: bool,
    fully_associative_cache: bool,
    #[bits(4, access = RO)]
    reserved0: u8,
    #[bits(12)]
    maximum_number_of_addressable_ids_for_logical_processors_sharing_this_cache: u16,
    #[bits(6)]
    maximum_number_of_addressable_ids_for_processor_cores_in_the_physical_package: u8,
}

#[bitfield(u32)]
struct Ebx {
    #[bits(12)]
    system_coherency_line_size: u16,
    #[bits(10)]
    physical_line_partitions: u16,
    #[bits(10)]
    ways_of_associativity: u16,
}

#[bitfield(u32)]
struct Ecx {
    #[bits(32)]
    number_of_bits: u32,
}

#[bitfield(u32)]
struct Edx {
    write_back_invalidate_invalidate: bool,
    cache_inclusiveness: bool,
    complex_cache_indexing: bool,
    #[bits(29, access = RO)]
    reserved0: u32,
}

