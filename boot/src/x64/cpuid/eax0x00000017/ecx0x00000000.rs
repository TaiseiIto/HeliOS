//! # CPUID
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol. 2A 3-217

use {super::super::Return, bitfield_struct::bitfield};

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
        Self { eax, ebx, ecx, edx }
    }
}

#[bitfield(u32)]
struct Eax {
    max_socid_index: u32,
}

#[bitfield(u32)]
struct Ebx {
    soc_vendor_id: u16,
    is_vendor_scheme: bool,
    #[bits(15)]
    __: u32,
}

#[bitfield(u32)]
struct Ecx {
    project_id: u32,
}

#[bitfield(u32)]
struct Edx {
    stepping_id: u32,
}
