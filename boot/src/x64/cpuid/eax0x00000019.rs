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
pub struct Eax0x00000019 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
}

impl Eax0x00000019 {
    pub fn get(eax0x00000000: &Eax0x00000000) -> Option<Self> {
        let eax: u32 = 0x00000019;
        let ecx: u32 = 0x00000000;
        (eax <= eax0x00000000.max_eax()).then(|| {
            let eax0x00000019 = Return::get(eax, ecx);
            let eax: Eax = eax0x00000019.eax().into();
            let ebx: Ebx = eax0x00000019.ebx().into();
            let ecx: Ecx = eax0x00000019.ecx().into();
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
    key_locker_restriction_of_cplo_only_supported: bool,
    key_locker_restriction_of_no_encrypt_supported: bool,
    key_locker_restriction_of_no_decrypt_supported: bool,
    #[bits(29, access = RO)]
    reserved0: u32,
}

#[bitfield(u32)]
struct Ebx {
    aeskle: bool,
    #[bits(access = RO)]
    reserved0: bool,
    the_aes_wide_key_locker_instructions_are_supported: bool,
    #[bits(access = RO)]
    reserved1: bool,
    the_platform_supports_the_key_locker_msrs_andbacking_up_the_internal_wrapping_key: bool,
    #[bits(27, access = RO)]
    reserved2: u32,
}

#[bitfield(u32)]
struct Ecx {
    the_nobackup_parameter_to_loadiwkey_is_supported: bool,
    keysource_encoding_of_1_is_supported: bool,
    #[bits(30, access = RO)]
    reserved0: u32,
}

