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
    edx: Edx,
}

impl Ecx0x00000000 {
    pub fn get(eax: u32) -> Self {
        let ecx: u32 = 0x00000001;
        let ecx0x00000000 = Return::get(eax, ecx);
        let eax: Eax = ecx0x00000000.eax().into();
        let ebx: Ebx = ecx0x00000000.ebx().into();
        let edx: Edx = ecx0x00000000.edx().into();
        Self {
            eax,
            ebx,
            edx,
        }
    }
}

#[bitfield(u32)]
struct Eax {
    sgx1: bool,
    sgx2: bool,
    #[bits(3, access = RO)]
    reserved0: u8,
    intel_sgx_supports_enclv_instruction_leaves_eincvirtchild_edecvirtchild_and_esetcontext: bool,
    intel_sgx_supports_encls_instruction_leaves_etrackc_erdinfo_eldbc_and_elduc: bool,
    intel_sgx_supports_enclu_instruction_leaf_everifyreport2: bool,
    #[bits(2, access = RO)]
    reserved1: u8,
    intel_sgx_supports_encls_instruction_leaf_eupdatesvn: bool,
    intel_sgx_supoprts_enclu_instruction_leaf_edeccssa: bool,
    #[bits(20, access = RO)]
    reserved2: u32,
}

#[bitfield(u32)]
struct Ebx {
    miscselect: u32,
}

#[bitfield(u32)]
struct Edx {
    maxenclavesize_not64: u8,
    maxenclavesize_64: u8,
    reserved0: u16,
}

