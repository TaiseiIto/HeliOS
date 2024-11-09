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
pub struct Eax0x0000001c {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
}

impl Eax0x0000001c {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x0000001c;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let eax0x0000001c = Return::get(eax, ecx);
            let eax: Eax = eax0x0000001c.eax().into();
            let ebx: Ebx = eax0x0000001c.ebx().into();
            let ecx: Ecx = eax0x0000001c.ecx().into();
            Self {
                eax,
                ebx,
                ecx,
            }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    supported_lbr_depth_values: u8,
    #[bits(22)]
    __: u32,
    deep_c_state_reset: bool,
    ip_value_contain_lip: bool,
}

#[bitfield(u32)]
struct Ebx {
    cpl_filtering_supported: bool,
    branch_filtering_supported: bool,
    call_stack_mode_supported: bool,
    #[bits(29)]
    __: u32,
}

#[bitfield(u32)]
struct Ecx {
    mispredict_bit_supported: bool,
    timed_lbrs_supported: bool,
    branch_type_field_supported: bool,
    #[bits(29)]
    __: u32,
}

