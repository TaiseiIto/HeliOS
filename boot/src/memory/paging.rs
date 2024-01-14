mod level4;

use crate::x64;

/// # Paging
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4 Paging
#[derive(Debug)]
pub enum Paging<'a> {
    Disable,
    Bit32,
    Pae,
    Level4 {
        pml4t: level4::Pml4t<'a>,
    },
    Level5,
}

impl Paging<'_> {
    pub fn get(cpuid: &Option<x64::Cpuid>) -> Self {
        let ia32_efer: Option<x64::msr::Ia32Efer> = x64::msr::Ia32Efer::get(cpuid);
        let cr0 = x64::control::Register0::get();
        let cr3 = x64::control::Register3::get();
        let cr4 = x64::control::Register4::get();
        if !cr0.paging_is_enabled() {
            Self::Disable
        } else if cr4.bit32_paging_is_used() {
            Self::Bit32
        } else if ia32_efer
            .expect("Can't get a paging structure.")
            .pae_paging_is_used() {
            Self::Pae
        } else if cr4.level4_paging_is_used() {
            let pml4t: level4::Pml4t = cr3.into();
            Self::Level4 {
                pml4t,
            }
        } else {
            Self::Level5
        }
    }
}

