//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::{
        Ecx0x00000000,
        super::Return,
    },
};

#[derive(Debug)]
pub struct Ecx0x00000002 {
    #[allow(dead_code)]
    edx: Edx,
}

impl Ecx0x00000002 {
    pub fn get(eax: u32, ecx0x00000000: &Ecx0x00000000) -> Option<Self> {
        let ecx: u32 = 0x00000002;
        (ecx <= ecx0x00000000.max_ecx()).then(|| {
            let ecx0x00000002 = Return::get(eax, ecx);
            let edx: Edx = ecx0x00000002.edx().into();
            Self {
                edx,
            }
        })
    }
}

#[bitfield(u32)]
struct Edx {
    psfd: bool,
    ipred_ctrl: bool,
    rrsba_ctrl: bool,
    ddpd_u: bool,
    bhi_ctrl: bool,
    mcdt_no: bool,
    #[bits(26)]
    __: u32,
}

