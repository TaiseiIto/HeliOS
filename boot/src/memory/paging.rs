mod level4;

use crate::asm;

/// # Paging
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4 Paging
#[derive(Debug)]
pub enum Paging {
    Disable,
    Bit32,
    Pae,
    Level4 {
        pml4_table: level4::Pml4Table,
    },
    Level5,
}

impl Paging {
    pub fn get(ia32_efer: &Option<asm::msr::Ia32Efer>) -> Self {
        let cr0 = asm::control::Register0::get();
        let cr3 = asm::control::Register3::get();
        let cr4 = asm::control::Register4::get();
        if !cr0.paging_is_enabled() {
            Self::Disable
        } else if cr4.bit32_paging_is_used() {
            Self::Bit32
        } else if ia32_efer
            .as_ref()
            .expect("Can't get a paging structure.")
            .pae_paging_is_used() {
            Self::Pae
        } else if cr4.level4_paging_is_used() {
            let cr3: level4::Cr3 = cr3.into();
            let pml4_table: level4::Pml4Table = cr3.into();
            Self::Level4 {
                pml4_table,
            }
        } else {
            Self::Level5
        }
    }
}

