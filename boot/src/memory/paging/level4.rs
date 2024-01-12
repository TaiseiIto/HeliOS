//! # 4-Level Paging
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4.5 4-Level Paging and 5-Level Paging

use {
    bitfield_struct::bitfield,
    crate::asm,
};

/// # 4-Level Paging CR3
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
pub struct Cr3 {
    #[bits(3, access = RO)]
    reserved0: u8,
    pwt: bool,
    pcd: bool,
    #[bits(7, access = RO)]
    reserved1: u8,
    #[bits(36)]
    pml4_table: u64,
    reserved2: u16,
}

impl From<asm::control::Register3> for Cr3 {
    fn from(cr3: asm::control::Register3) -> Self {
        let cr3: u64 = cr3.into();
        cr3.into()
    }
}

