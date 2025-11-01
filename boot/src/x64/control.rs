//! # Control Registers
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 2.5 Control Registers

use {crate::memory, bitfield_struct::bitfield, core::arch::asm};

#[bitfield(u64)]
pub struct Register0 {
    pe: bool,
    mp: bool,
    em: bool,
    ts: bool,
    et: bool,
    ne: bool,
    #[bits(10)]
    __: u16,
    wp: bool,
    __: bool,
    am: bool,
    #[bits(10)]
    __: u16,
    nw: bool,
    cd: bool,
    pg: bool,
    __: u32,
}

impl Register0 {
    #[inline(never)]
    pub fn get() -> Self {
        let mut cr0: u64;
        unsafe {
            asm!(
                "mov {0}, cr0",
                out(reg) cr0,
            );
        }
        cr0.into()
    }

    pub fn paging_is_enabled(&self) -> bool {
        self.pe() && self.pg()
    }
}

#[bitfield(u64)]
pub struct Register2 {
    page_fault_linear_address: u64,
}

impl Register2 {
    #[inline(never)]
    pub fn get() -> Self {
        let mut cr2: u64;
        unsafe {
            asm!(
                "mov {0}, cr2",
                out(reg) cr2,
            );
        }
        cr2.into()
    }
}

#[bitfield(u64)]
pub struct Register3 {
    #[bits(3)]
    __: u8,
    pwt: bool,
    pcd: bool,
    #[bits(7)]
    __: u8,
    #[bits(52)]
    page_directory_base: u64,
}

impl Register3 {
    #[inline(never)]
    pub fn get() -> Self {
        let mut cr3: u64;
        unsafe {
            asm!(
                "mov {0}, cr3",
                out(reg) cr3,
            );
        }
        cr3.into()
    }

    pub fn get_paging_structure<T>(&self) -> &T
    where
        T: memory::paging::TopTable,
    {
        let page_directory_base: u64 =
            self.page_directory_base() << Self::PAGE_DIRECTORY_BASE_OFFSET;
        let page_directory_base: *const T = page_directory_base as *const T;
        unsafe { &*page_directory_base }
    }

    #[inline(never)]
    pub fn set(&self) {
        let cr3: u64 = (*self).into();
        unsafe {
            asm!(
                "mov cr3, {0}",
                in(reg) cr3,
            );
        }
    }

    pub fn with_paging_structure<T>(self, page_directory_base: &T) -> Self
    where
        T: memory::paging::TopTable,
    {
        let page_directory_base: *const T = page_directory_base as *const T;
        let page_directory_base: u64 = page_directory_base as u64;
        let page_directory_base: u64 = page_directory_base >> Self::PAGE_DIRECTORY_BASE_OFFSET;
        self.with_page_directory_base(page_directory_base)
    }
}

#[bitfield(u64)]
pub struct Register4 {
    vme: bool,
    pvi: bool,
    tsd: bool,
    de: bool,
    pse: bool,
    pae: bool,
    mce: bool,
    pge: bool,
    pce: bool,
    osfxsr: bool,
    osxmmexcpt: bool,
    umip: bool,
    la57: bool,
    vmxe: bool,
    smxe: bool,
    __: bool,
    fsgsbase: bool,
    pcide: bool,
    oscsafe: bool,
    kl: bool,
    smep: bool,
    smap: bool,
    pke: bool,
    cep: bool,
    pks: bool,
    uintr: bool,
    #[bits(38)]
    __: u64,
}

impl Register4 {
    #[inline(never)]
    pub fn get() -> Self {
        let mut cr4: u64;
        unsafe {
            asm!(
                "mov {0}, cr4",
                out(reg) cr4,
            );
        }
        cr4.into()
    }

    pub fn bit32_paging_is_used(&self) -> bool {
        !self.pae()
    }

    pub fn level4_paging_is_used(&self) -> bool {
        !self.la57()
    }
}
