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
pub struct Eax0x00000015 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
}

impl Eax0x00000015 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000015;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x00000000.max_eax() {
            let eax0x00000015 = Return::get(eax, ecx);
            let eax: Eax = eax0x00000015.eax().into();
            let ebx: Ebx = eax0x00000015.ebx().into();
            let ecx: Ecx = eax0x00000015.ecx().into();
            Some(Self {
                eax,
                ebx,
                ecx,
            })
        } else {
            None
        }
    }
}

#[bitfield(u32)]
struct Eax {
    an_assigned_integer_which_is_the_denominator_of_the_tsc_core_crystal_clock_ratio: u32,
}

#[bitfield(u32)]
struct Ebx {
    an_assigned_integer_which_is_the_numerator_of_the_tsc_core_crystal_clock_ratio: u32,
}

#[bitfield(u32)]
struct Ecx {
    an_assigned_integer_which_is_the_nominal_frequency_of_the_core_crystal_clock_in_hz: u32,
}

