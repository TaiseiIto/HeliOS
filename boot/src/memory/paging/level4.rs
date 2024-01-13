//! # 4-Level Paging
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4.5 4-Level Paging and 5-Level Paging

use {
    bitfield_struct::bitfield,
    core::mem,
    crate::asm,
    super::super::KIB,
};

const TABLE_SIZE: usize = 4 * KIB;
const PML4_TABLE_LENGTH: usize = TABLE_SIZE / mem::size_of::<Pml4e>();

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
    address_of_pml4_table: u64,
    #[bits(access = RO)]
    reserved2: u16,
}

impl Cr3 {
    fn pml4_table(&self) -> &mut [Pml4e; PML4_TABLE_LENGTH] {
        let pml4_table: u64 = self.address_of_pml4_table() << Self::ADDRESS_OF_PML4_TABLE_OFFSET;
        let pml4_table: *mut [Pml4e; PML4_TABLE_LENGTH] = pml4_table as *mut [Pml4e; PML4_TABLE_LENGTH];
        unsafe {
            &mut *pml4_table
        }
    }
}

impl From<asm::control::Register3> for Cr3 {
    fn from(cr3: asm::control::Register3) -> Self {
        let cr3: u64 = cr3.into();
        cr3.into()
    }
}

/// # 4-Level Paging PML4E
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[derive(Debug)]
pub struct Pml4Table {
    cr3: Cr3,
}

impl From<Cr3> for Pml4Table {
    fn from(cr3: Cr3) -> Self {
        Self {
            cr3,
        }
    }
}

/// # 4-Level Paging PML4E
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct Pml4e {
    p: bool,
    rw: bool,
    us: bool,
    pwt: bool,
    pcd: bool,
    a: bool,
    #[bits(5, access = RO)]
    reserved0: u8,
    r: bool,
    #[bits(36)]
    address_of_pml4_table: u64,
    #[bits(15, access = RO)]
    reserved1: u16,
    xd: bool,
}
