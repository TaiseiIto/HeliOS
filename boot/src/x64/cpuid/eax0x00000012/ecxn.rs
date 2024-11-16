//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::super::Return,
};

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
    pub fn get(eax: u32, ecx: u32) -> Option<Self> {
        let ecxn = Return::get(eax, ecx);
        let eax: Eax = ecxn.eax().into();
        match eax.sub_leaf_type() {
            0 => None,
            _ => {
                let ebx: Ebx = ecxn.ebx().into();
                let ecx: Ecx = ecxn.ecx().into();
                let edx: Edx = ecxn.edx().into();
                Some(Self {
                    eax,
                    ebx,
                    ecx,
                    edx,
                })
            },
        }
    }
}

#[bitfield(u32)]
struct Eax {
    #[bits(4)]
    sub_leaf_type: u8,
    __: u8,
    #[bits(20)]
    bits_31_12_of_the_physical_address_of_the_base_of_the_epc_section: u32,
}

#[bitfield(u32)]
struct Ebx {
    #[bits(20)]
    bits_51_32_of_the_physical_address_of_the_base_of_the_epc_section: u32,
    #[bits(12)]
    __: u16,
}

#[bitfield(u32)]
struct Ecx {
    #[bits(4)]
    epc_section_property_encoding: u8,
    __: u8,
    #[bits(20)]
    bits_31_12_of_the_size_of_the_corresponding_epc_section_within_the_processor_reserved_memory: u32,
}

#[bitfield(u32)]
struct Edx {
    #[bits(20)]
    bits_51_32_of_the_size_of_the_corresponding_epc_section_within_the_processor_reserved_memory: u32,
    #[bits(12)]
    __: u16,
}

