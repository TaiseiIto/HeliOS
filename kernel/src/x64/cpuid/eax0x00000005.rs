//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    super::{Eax0x00000000, Return},
    bitfield_struct::bitfield,
};

#[derive(Debug)]
pub struct Eax0x00000005 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Eax0x00000005 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000005;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let eax0x00000005 = Return::get(eax, ecx);
            let eax: Eax = eax0x00000005.eax().into();
            let ebx: Ebx = eax0x00000005.ebx().into();
            let ecx: Ecx = eax0x00000005.ecx().into();
            let edx: Edx = eax0x00000005.edx().into();
            Self { eax, ebx, ecx, edx }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    smallest_monitor_line_size_in_bytes: u16,
    __: u16,
}

#[bitfield(u32)]
struct Ebx {
    largest_monitor_line_size_in_bytes: u16,
    __: u16,
}

#[bitfield(u32)]
struct Ecx {
    enumeration_of_monitor_mwait_extensions_beyond_eax_and_ebx_registers_supported: bool,
    supports_treating_interrupts_as_break_event_for_mwait_even_when_interrupts_disabled: bool,
    #[bits(30)]
    __: u32,
}

#[bitfield(u32)]
struct Edx {
    #[bits(4)]
    number_of_c0_sub_c_states_supported_using_mwait: u8,
    #[bits(4)]
    number_of_c1_sub_c_states_supported_using_mwait: u8,
    #[bits(4)]
    number_of_c2_sub_c_states_supported_using_mwait: u8,
    #[bits(4)]
    number_of_c3_sub_c_states_supported_using_mwait: u8,
    #[bits(4)]
    number_of_c4_sub_c_states_supported_using_mwait: u8,
    #[bits(4)]
    number_of_c5_sub_c_states_supported_using_mwait: u8,
    #[bits(4)]
    number_of_c6_sub_c_states_supported_using_mwait: u8,
    #[bits(4)]
    number_of_c7_sub_c_states_supported_using_mwait: u8,
}
