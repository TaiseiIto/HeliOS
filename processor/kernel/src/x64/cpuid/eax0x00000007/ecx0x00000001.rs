//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    super::{super::Return, Ecx0x00000000},
    bitfield_struct::bitfield,
};

#[derive(Debug)]
pub struct Ecx0x00000001 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Ecx0x00000001 {
    pub fn get(eax: u32, ecx0x00000000: &Ecx0x00000000) -> Option<Self> {
        let ecx: u32 = 0x00000001;
        (ecx <= ecx0x00000000.max_ecx()).then(|| {
            let ecx0x00000001 = Return::get(eax, ecx);
            let eax: Eax = ecx0x00000001.eax().into();
            let ebx: Ebx = ecx0x00000001.ebx().into();
            let edx: Edx = ecx0x00000001.edx().into();
            Self { eax, ebx, edx }
        })
    }
}

#[bitfield(u32)]
struct Eax {
    #[bits(4)]
    __: u8,
    avx_vnni: bool,
    avx512_bf16: bool,
    #[bits(4)]
    __: u8,
    supports_fast_zero_length_rep_movsb: bool,
    supports_fast_short_rep_stosb: bool,
    supports_fast_short_rep_cmpsb_rep_scasb: bool,
    #[bits(9)]
    __: u16,
    hreset: bool,
    #[bits(7)]
    __: u8,
    invd_disable_post_bios_done: bool,
    __: bool,
}

#[bitfield(u32)]
struct Ebx {
    enumerates_the_presence_of_the_ia32_ppin_and_ia32_ppin_ctl_msrs: bool,
    #[bits(31)]
    __: u32,
}

#[bitfield(u32)]
struct Edx {
    #[bits(18)]
    __: u32,
    cet_sss: bool,
    #[bits(13)]
    __: u16,
}
