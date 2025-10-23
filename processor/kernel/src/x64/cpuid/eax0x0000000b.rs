//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    super::{Eax0x00000000, Return},
    bitfield_struct::bitfield,
};

#[derive(Debug)]
pub struct Eax0x0000000b {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Eax0x0000000b {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000000b;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let eax0x0000000b = Return::get(eax, ecx);
            let eax: Eax = eax0x0000000b.eax().into();
            let ebx: Ebx = eax0x0000000b.ebx().into();
            let ecx: Ecx = eax0x0000000b.ecx().into();
            let edx: Edx = eax0x0000000b.edx().into();
            Self { eax, ebx, ecx, edx }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    #[bits(5)]
    the_number_of_bits_that_the_x2apic_id_must_be_shifted_to_the_right_to_address_instances_of_the_next_higher_scoped_domain:
        u8,
    #[bits(27)]
    __: u32,
}

#[bitfield(u32)]
struct Ebx {
    the_number_of_logical_processors_across_all_instances_of_this_domain_within_the_next_higher_scoped_domain:
        u16,
    __: u16,
}

#[bitfield(u32)]
struct Ecx {
    the_input_ecx_sub_leaf_index: u8,
    domain_type: u8,
    __: u16,
}

#[bitfield(u32)]
struct Edx {
    x2apic_id_of_the_current_logical_processor: u32,
}
