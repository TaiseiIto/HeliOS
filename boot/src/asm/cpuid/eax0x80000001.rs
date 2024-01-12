//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {
    bitfield_struct::bitfield,
    super::{
        Eax0x80000000,
        Return,
    },
};

#[derive(Debug)]
pub struct Eax0x80000001 {
    #[allow(dead_code)]
    eax: Eax,
    #[allow(dead_code)]
    ecx: Ecx,
    edx: Edx,
}

impl Eax0x80000001 {
    pub fn get(eax0x80000000: &Eax0x80000000) -> Option<Self> {
        let eax: u32 = 0x80000001;
        let ecx: u32 = 0x00000000;
        if eax <= eax0x80000000.max_eax() {
            let eax0x80000001 = Return::get(eax, ecx);
            let eax: Eax = eax0x80000001.eax().into();
            let ecx: Ecx = eax0x80000001.ecx().into();
            let edx: Edx = eax0x80000001.edx().into();
            Some(Self {
                eax,
                ecx,
                edx,
            })
        } else {
            None
        }
    }

    /// # CPUID
    /// ## References
    /// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 4 2-63
    /// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-239
    pub fn ia32_efer_is_supported(&self) -> bool {
        self.edx.execute_disable_bit_available() || self.edx.intel64_architecture_available()
    }
}

#[bitfield(u32)]
struct Eax {
    extended_processor_signature_and_feature_bits: u32,
}

#[bitfield(u32)]
struct Ecx {
    lahf_sahf_available_in_64bit_mode: bool,
    #[bits(4, access = RO)]
    reserved0: u8,
    lzcnt: bool,
    #[bits(2, access = RO)]
    reserved1: u8,
    prefetchw: bool,
    #[bits(23, access = RO)]
    reserved2: u32,
}

#[bitfield(u32)]
struct Edx {
    #[bits(11, access = RO)]
    reserved0: u16,
    syscall_sysret: bool,
    #[bits(access = RO)]
    reserved1: u8,
    execute_disable_bit_available: bool,
    #[bits(5, access = RO)]
    reserved2: u8,
    pages_1gb_are_available: bool,
    rdtscp_and_ia32_tsc_aux_are_available: bool,
    #[bits(access = RO)]
    reserved3: bool,
    intel64_architecture_available: bool,
    #[bits(2, access = RO)]
    reserved4: u8,
}

