mod level4;

use {
    core::ops::Range,
    crate::{
        com2_print,
        com2_println,
        x64,
    },
};

/// # Paging
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4 Paging
#[derive(Debug)]
pub enum Paging {
    #[allow(dead_code)]
    Disable,
    #[allow(dead_code)]
    Bit32,
    #[allow(dead_code)]
    Pae,
    #[allow(dead_code)]
    Level4 {
        interface: level4::Interface,
    },
    #[allow(dead_code)]
    Level5,
}

impl Paging {
    #[allow(dead_code)]
    pub fn debug(&self, vaddr: usize) {
        com2_println!("Begin paging information vaddr {:#x?}", vaddr);
        match self {
            Self::Disable => {},
            Self::Bit32 => {},
            Self::Pae => {},
            Self::Level4 {
                interface
            } => {
                interface.debug(vaddr)
            },
            Self::Level5 => {},
        }
        com2_println!("End paging information vaddr {:#x?}", vaddr);
    }

    pub fn higher_half_range(&self) -> Range<u128> {
        match self {
            Self::Disable => panic!("Unimplemented!"),
            Self::Bit32 => panic!("Unimplemented!"),
            Self::Pae => panic!("Unimplemented!"),
            Self::Level4 {
                interface
            } => {
                interface.higher_half_range()
            },
            Self::Level5 => panic!("Unimplemented!"),
        }
    }

    #[allow(dead_code)]
    pub fn set(&self) {
        match self {
            Self::Disable => {},
            Self::Bit32 => {},
            Self::Pae => {},
            Self::Level4 {
                interface
            } => {
                interface.set()
            },
            Self::Level5 => {},
        }
    }

    pub fn set_page(&mut self, vaddr: usize, paddr: usize, present: bool, writable: bool, executable: bool) {
        match self {
            Self::Disable => {},
            Self::Bit32 => {},
            Self::Pae => {},
            Self::Level4 {
                interface
            } => {
                interface.set_page(vaddr, paddr, present, writable, executable)
            },
            Self::Level5 => {},
        }
    }

    pub fn vaddr2paddr<T>(&self, vaddr: &T) -> Option<usize> {
        let vaddr: *const T = vaddr as *const T;
        let vaddr: usize = vaddr as usize;
        match self {
            Self::Disable => None,
            Self::Bit32 => None,
            Self::Pae => None,
            Self::Level4 {
                interface
            } => {
                interface.vaddr2paddr(vaddr)
            },
            Self::Level5 => None,
        }
    }
}

