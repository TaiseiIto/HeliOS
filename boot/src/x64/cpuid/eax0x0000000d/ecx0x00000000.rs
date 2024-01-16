//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::super::Return,
};

#[derive(Debug)]
pub struct Ecx0x00000000 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Ecx0x00000000 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000000;
        let ecx0x00000000 = Return::get(eax, ecx);
        let eax: Eax = ecx0x00000000.eax().into();
        let ebx: Ebx = ecx0x00000000.ebx().into();
        let ecx: Ecx = ecx0x00000000.ecx().into();
        let edx: Edx = ecx0x00000000.edx().into();
        Self {
            eax,
            ebx,
            ecx,
            edx,
        }
    }

    pub fn xcr0_n_is_valid(&self, n: u32) -> bool {
        if (0..u32::BITS).contains(&n) {
            let eax: u32 = self.eax.into();
            eax & (1 << n) != 0
        } else if (u32::BITS..2 * u32::BITS).contains(&n) {
            let edx: u32 = self.edx.into();
            edx & (1 << (n - u32::BITS)) != 0
        } else {
            panic!("Invalid XCR0 index {}.", n)
        }
    }
}

#[bitfield(u32)]
struct Eax {
    x87_state: bool,
    sse_state: bool,
    avx_state: bool,
    #[bits(2)]
    mpx_state: u8,
    #[bits(3)]
    avx512_state: u8,
    used_for_ia32_xdd0: bool,
    pkru_state: bool,
    #[bits(7)]
    used_for_ia32_xdd1: u8,
    tilecfg_state: bool,
    tiledata_state: bool,
    #[bits(13, access = RO)]
    reserved0: u16,
}

#[bitfield(u32)]
struct Ebx {
    maximum_size_required_by_enabled_features_in_xcr0: u32,
}

#[bitfield(u32)]
struct Ecx {
    maximum_size_of_the_xsave_xrstor_save_area_required_by_all_supported_features_in_the_processor: u32,
}

#[bitfield(u32)]
struct Edx {
    reports_the_supported_bits_of_the_upper_32_bits_of_xcr0: u32,
}

