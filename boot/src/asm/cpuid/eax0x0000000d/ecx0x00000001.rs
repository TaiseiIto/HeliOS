//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::super::Return,
};

#[derive(Debug)]
pub struct Ecx0x00000001 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ebx: Ebx,
    #[allow(dead_code)]
    ecx: Ecx,
    #[allow(dead_code)]
    edx: Edx,
}

impl Ecx0x00000001 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000000;
        let ecx0x00000001 = Return::get(eax, ecx);
        let eax: Eax = ecx0x00000001.eax().into();
        let ebx: Ebx = ecx0x00000001.ebx().into();
        let ecx: Ecx = ecx0x00000001.ecx().into();
        let edx: Edx = ecx0x00000001.edx().into();
        Self {
            eax,
            ebx,
            ecx,
            edx,
        }
    }

    pub fn ia32_xss_n_is_valid(&self, n: u32) -> bool {
        if (0..u32::BITS).contains(&n) {
            let ecx: u32 = self.ecx.into();
            ecx & (1 << n) != 0
        } else if (u32::BITS..2 * u32::BITS).contains(&n) {
            let edx: u32 = self.edx.into();
            edx & (1 << (n - u32::BITS)) != 0
        } else {
            panic!("Can't get IA32_XSS.n validity.")
        }
    }
}

#[bitfield(u32)]
struct Eax {
    xsaveopt: bool,
    supports_xsavec_and_the_compacted_form_of_xrstor: bool,
    supports_xgetbv_with_ecx_1: bool,
    supports_xsaves_xrstors_and_ia32_xdd: bool,
    supports_xfd: bool,
    #[bits(27, access = RO)]
    reserved0: u32,
}

#[bitfield(u32)]
struct Ebx {
    the_size_in_bytes_of_the_xsave_area_containing_all_states_enabled_by_xcr0_ia32_xss: u32,
}

#[bitfield(u32)]
struct Ecx {
    used_for_xcr0_0: u8,
    pt_state: bool,
    used_for_xcr0_1: bool,
    pasid_state: bool,
    cet_user_state: bool,
    cet_supervisor_state: bool,
    hdc_state: bool,
    uintr_state: bool,
    lbr_state: bool,
    hwp_state: bool,
    #[bits(2)]
    used_for_xcr0_2: u8,
    #[bits(13, access = RO)]
    reserved0: u16,
}

#[bitfield(u32)]
struct Edx {
    reports_the_supported_bits_of_the_upper_32_bits_of_the_ia32_xss_msr: u32,
}

