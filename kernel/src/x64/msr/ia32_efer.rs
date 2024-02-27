//! # IA32_EFER
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63

use {
    bitfield_struct::bitfield,
    super::{
        rdmsr,
        wrmsr,
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
    const ECX: u32 = 0xc0000080;

    pub fn enable_execute_disable_bit(cpuid: &Option<Cpuid>) -> bool {
        cpuid
            .as_ref()
            .map_or(false, |cpuid| cpuid.execute_disable_bit_available())
            .then(|| Self::get(cpuid)
                .map_or(false, |ia32_efer| {
                    ia32_efer
                        .with_nxe(true)
                        .set();
                    true
                }))
            .unwrap_or(false)
    }

    pub fn enable_system_call_enable_bit(cpuid: &Option<Cpuid>) -> bool {
        Self::get(cpuid)
            .map_or(false, |ia32_efer| {
                ia32_efer
                    .with_sce(true)
                    .with_lma(true)
                    .set();
                true
            })
    }

    pub fn get(cpuid: &Option<Cpuid>) -> Option<Self> {
        cpuid
            .as_ref()
            .and_then(|cpuid| cpuid
                .ia32_efer_is_supported()
                .then(|| {
                    let ia32_efer: u64 = rdmsr(Self::ECX);
                    ia32_efer.into()
                }))
    }

    pub fn pae_paging_is_used(&self) -> bool {
        !self.lme()
    }

    pub fn set(self) {
        let ia32_efer: u64 = self.into();
        wrmsr(Self::ECX, ia32_efer);
    }
}

