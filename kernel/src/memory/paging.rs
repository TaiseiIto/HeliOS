mod level4;

use {
    crate::{com2_println, x64},
    core::ops::Range,
};

/// # Paging
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4 Paging
#[derive(Clone, Debug)]
pub enum Paging {
    Disable,
    Bit32,
    Pae,
    Level4 { controller: level4::Controller },
    Level5,
}

impl Paging {
    pub fn cr3(&self) -> x64::control::Register3 {
        match self {
            Self::Disable => unimplemented!(),
            Self::Bit32 => unimplemented!(),
            Self::Pae => unimplemented!(),
            Self::Level4 { controller } => controller.cr3(),
            Self::Level5 => unimplemented!(),
        }
    }

    pub fn debug(&self, vaddr: usize) {
        com2_println!("Begin paging information vaddr {:#x?}", vaddr);
        match self {
            Self::Disable => unimplemented!(),
            Self::Bit32 => unimplemented!(),
            Self::Pae => unimplemented!(),
            Self::Level4 { controller } => controller.debug(vaddr),
            Self::Level5 => unimplemented!(),
        }
        com2_println!("End paging information vaddr {:#x?}", vaddr);
    }

    pub fn higher_half_range(&self) -> Range<u128> {
        match self {
            Self::Disable => unimplemented!(),
            Self::Bit32 => unimplemented!(),
            Self::Pae => unimplemented!(),
            Self::Level4 { controller } => controller.higher_half_range(),
            Self::Level5 => unimplemented!(),
        }
    }

    pub fn set(&self) {
        match self {
            Self::Disable => unimplemented!(),
            Self::Bit32 => unimplemented!(),
            Self::Pae => unimplemented!(),
            Self::Level4 { controller } => controller.set(),
            Self::Level5 => unimplemented!(),
        }
    }

    pub fn set_page(
        &mut self,
        vaddr: usize,
        paddr: usize,
        present: bool,
        writable: bool,
        executable: bool,
    ) {
        match self {
            Self::Disable => unimplemented!(),
            Self::Bit32 => unimplemented!(),
            Self::Pae => unimplemented!(),
            Self::Level4 { controller } => {
                controller.set_page(vaddr, paddr, present, writable, executable)
            }
            Self::Level5 => unimplemented!(),
        }
    }

    pub fn table(&self) -> &[u8] {
        match self {
            Self::Disable => unimplemented!(),
            Self::Bit32 => unimplemented!(),
            Self::Pae => unimplemented!(),
            Self::Level4 { controller } => controller.pml4t(),
            Self::Level5 => unimplemented!(),
        }
    }

    pub fn vaddr2paddr<T>(&self, vaddr: &T) -> Option<usize> {
        let vaddr: *const T = vaddr as *const T;
        let vaddr: usize = vaddr as usize;
        match self {
            Self::Disable => unimplemented!(),
            Self::Bit32 => unimplemented!(),
            Self::Pae => unimplemented!(),
            Self::Level4 { controller } => controller.vaddr2paddr(vaddr),
            Self::Level5 => unimplemented!(),
        }
    }
}
