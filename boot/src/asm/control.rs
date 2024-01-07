//! # Control Registers
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 2.5 Control Registers

use {
    bitfield_struct::bitfield,
    core::arch::asm,
};

#[bitfield(u32)]
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
}

impl Register0 {
    pub fn get() -> Self {
        let mut cr0: u32;
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

    pub fn bit32_paging_is_enabled(&self) -> bool {
        !self.pae()
    }
}

