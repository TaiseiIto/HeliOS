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
pub struct Eax0x00000020 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
}

impl Eax0x00000020 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000020;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let eax0x00000020 = Return::get(eax, ecx);
            let eax: Eax = eax0x00000020.eax().into();
            let ebx: Ebx = eax0x00000020.ebx().into();
            Self {
                eax,
                ebx,
            }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    reports_the_maximum_number_of_sub_leaves_that_are_supported_in_leaf_20h: u32,
}

#[bitfield(u32)]
struct Ebx {
    indicates_support_for_both_hresets_eax0_parameter_and_ia32_hreset_enable0_set_by_the_os_to_enable_reset_of_intel_thread_director_history: bool,
    #[bits(31)]
    __: u32,
}

