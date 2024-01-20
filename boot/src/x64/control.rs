//! # Control Registers
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 2.5 Control Registers

use {
    bitfield_struct::bitfield,
    core::arch::asm,
};

#[bitfield(u64)]
pub struct Register0 {
    pe: bool,
    mp: bool,
    em: bool,
    ts: bool,
    et: bool,
    ne: bool,
    #[bits(10, access = RO)]
    reserved0: u16,
    wp: bool,
    #[bits(access = RO)]
    reserved1: bool,
    am: bool,
    #[bits(10, access = RO)]
    reserved2: u16,
    nw: bool,
    cd: bool,
    pg: bool,
    #[bits(32, access = RO)]
    reserved3: u32,
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
    #[bits(3, access = RO)]
    reserved0: u8,
    pwt: bool,
    pcd: bool,
    #[bits(7, access = RO)]
    reserved1: u8,
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

    pub fn get_paging_structure<T>(&self) -> &T {
        let page_directory_base: u64 = self.page_directory_base() << Self::PAGE_DIRECTORY_BASE_OFFSET;
        let page_directory_base: *const T = page_directory_base as *const T;
        unsafe {
            &*page_directory_base
        }
    }

    pub fn with_paging_structure<T>(self, page_directory_base: &T) -> Self {
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
    #[bits(access = RO)]
    reserved0: bool,
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
    #[bits(38, access = RO)]
    reserved1: u64,
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

