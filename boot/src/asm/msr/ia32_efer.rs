//! # IA32_EFER
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63

use {
    bitfield_struct::bitfield,
    super::{
        rdmsr,
        super::Cpuid,
    },
};

/// # IA32_EFER
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63
#[bitfield(u64)]
pub struct Ia32Efer {
    sce: bool,
    #[bits(7, access = RO)]
    reserved0: u8,
    lme: bool,
    #[bits(access = RO)]
    reserved1: bool,
    lma: bool,
    nxe: bool,
    #[bits(52, access = RO)]
    reserved2: u64,
}

impl Ia32Efer {
    pub fn get(cpuid: &Option<Cpuid>) -> Option<Self> {
        cpuid
            .as_ref()
            .and_then(|cpuid| if cpuid.ia32_efer_is_supported() {
                let ecx: u32 = 0xc0000080;
                let ia32_efer: u64 = rdmsr(ecx);
                let ia32_efer: Self = ia32_efer.into();
                Some(ia32_efer)
            } else {
                None
            })
    }

    pub fn pae_paging_is_used(&self) -> bool {
        !self.lme()
    }
}

