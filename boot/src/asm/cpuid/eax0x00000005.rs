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
pub struct Eax0x00000005 {
    eax: Eax,
    ebx: Ebx,
    ecx: Ecx,
    edx: Edx,
}

impl Eax0x00000005 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000005;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let eax0x00000005 = Return::get(eax, ecx);
            let eax: Eax = eax0x00000005.eax().into();
            let ebx: Ebx = eax0x00000005.ebx().into();
            let ecx: Ecx = eax0x00000005.ecx().into();
            let edx: Edx = eax0x00000005.edx().into();
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
pub struct Eax {
    smallest_monitor_line_size_in_bytes: u16,
    reserved0: u16,
}

#[bitfield(u32)]
pub struct Ebx {
    largest_monitor_line_size_in_bytes: u16,
    reserved0: u16,
}

#[bitfield(u32)]
pub struct Ecx {
    enumeration_of_monitor_mwait_extensions_beyond_eax_and_ebx_registers_supported: bool,
    supports_treating_interrupts_as_break_event_for_mwait_even_when_interrupts_disabled: bool,
    #[bits(30)]
    reserved0: u32,
}

#[bitfield(u32)]
pub struct Edx  {
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

