//! # IA32_EFER
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63

use {
    super::{super::Cpuid, rdmsr, wrmsr},
    bitfield_struct::bitfield,
};

/// # IA32_EFER
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.4 2-63
#[bitfield(u64)]
pub struct Ia32Efer {
    sce: bool,
    #[bits(7)]
    __: u8,
    lme: bool,
    __: bool,
    lma: bool,
    nxe: bool,
    #[bits(52)]
    __: u64,
}

impl Ia32Efer {
    const ECX: u32 = 0xc0000080;

    pub fn enable_execute_disable_bit(cpuid: &Cpuid) -> bool {
        cpuid
            .supports_execute_disable_bit()
            .then(|| {
                Self::get(cpuid).map_or(false, |ia32_efer| {
                    ia32_efer.with_nxe(true).set();
                    true
                })
            })
            .unwrap_or(false)
    }

    pub fn get(cpuid: &Cpuid) -> Option<Self> {
        cpuid.supports_ia32_efer().then(|| {
            let ia32_efer: u64 = rdmsr(Self::ECX);
            ia32_efer.into()
        })
    }

    pub fn pae_paging_is_used(&self) -> bool {
        !self.lme()
    }

    pub fn set(self) {
        let ia32_efer: u64 = self.into();
        wrmsr(Self::ECX, ia32_efer);
    }
}
