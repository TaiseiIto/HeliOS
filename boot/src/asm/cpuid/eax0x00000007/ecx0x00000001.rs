//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::{
        Ecx0x00000000,
        super::{
            Eax0x00000000,
            Return,
        },
    },
};

#[derive(Debug)]
pub struct Ecx0x00000001 {
    eax: Eax,
    ebx: Ebx,
    edx: Edx,
}

impl Ecx0x00000001 {
    pub fn get(eax: u32, ecx0x00000000: &Ecx0x00000000) -> Option<Self> {
        let ecx: u32 = 0x00000001;
        if ecx <= ecx0x00000000.max_ecx() {
            let ecx0x00000001 = Return::get(eax, ecx);
            let eax: Eax = ecx0x00000001.eax().into();
            let ebx: Ebx = ecx0x00000001.ebx().into();
            let edx: Edx = ecx0x00000001.edx().into();
            Some(Self {
                eax,
                ebx,
                edx,
            })
        } else {
            None
        }
    }
}

#[bitfield(u32)]
pub struct Eax {
    #[bits(4, access = RO)]
    reserved0: u8,
    avx_vnni: bool,
    avx512_bf16: bool,
    #[bits(4, access = RO)]
    reserved1: u8,
    supports_fast_zero_length_rep_movsb: bool,
    supports_fast_short_rep_stosb: bool,
    supports_fast_short_rep_cmpsb_rep_scasb: bool,
    #[bits(9, access = RO)]
    reserved2: u16,
    hreset: bool,
    #[bits(7, access = RO)]
    reserved3: u8,
    invd_disable_post_bios_done: bool,
    #[bits(access = RO)]
    reserved4: bool,
}

#[bitfield(u32)]
pub struct Ebx {
    enumerates_the_presence_of_the_ia32_ppin_and_ia32_ppin_ctl_msrs: bool,
    #[bits(31, access = RO)]
    reserved: u32,
}

#[bitfield(u32)]
pub struct Edx {
    #[bits(18, access = RO)]
    reserved: u32,
    cet_sss: bool,
    #[bits(13, access = RO)]
    reserved1: u16,
}

