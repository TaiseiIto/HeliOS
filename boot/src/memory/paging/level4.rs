//! # 4-Level Paging
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4.5 4-Level Paging and 5-Level Paging

use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
    },
    crate::asm,
};

const PAGE_SIZE: usize = 1 << 12;

/// # 4-Level Paging CR3
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
pub struct Cr3 {
    core: Cr3Core,
}

impl fmt::Debug for Cr3 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pml4_table: Vec<&Pml4e> = self.core
            .pml4_table()
            .iter()
            .filter(|pml4e| pml4e.present())
            .collect();
        formatter
            .debug_struct("Cr3")
            .field("core", &self.core)
            .field("PML4 table", &pml4_table)
            .finish()
    }
}

impl From<asm::control::Register3> for Cr3 {
    fn from(cr3: asm::control::Register3) -> Self {
        let cr3: u64 = cr3.into();
        let core: Cr3Core = cr3.into();
        Self {
            core,
        }
    }
}

/// # 4-Level Paging CR3
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct Cr3Core {
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

impl Cr3Core {
    const PML4_ENTRIES: usize = PAGE_SIZE / mem::size_of::<Pml4e>();

    pub fn pml4_table(&self) -> &[Pml4e; Self::PML4_ENTRIES] {
        let pml4_table: u64 = self.address_of_pml4_table() << Self::ADDRESS_OF_PML4_TABLE_OFFSET;
        let pml4_table: *const [Pml4e; Self::PML4_ENTRIES] = pml4_table as *const [Pml4e; Self::PML4_ENTRIES];
        unsafe {
            &*pml4_table
        }
    }
}

/// # PML4E
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
struct Pml4e {
    core: Pml4eCore,
}

impl Pml4e {
    fn present(&self) -> bool {
        self.core.p()
    }
}

impl fmt::Debug for Pml4e {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Pml4e")
            .field("core", &self.core)
            .finish()
    }
}

/// # PML4E
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct Pml4eCore {
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
    address_of_pdpt: u64,
    #[bits(15, access = RO)]
    reserved1: u16,
    xd: bool,
}

