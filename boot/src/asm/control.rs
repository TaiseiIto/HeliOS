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
    #[bits(10)]
    reserved0: u16,
    wp: bool,
    reserved1: bool,
    am: bool,
    #[bits(10)]
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
}

