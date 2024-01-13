//! # 4-Level Paging
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4.5 4-Level Paging and 5-Level Paging

use {
    alloc::collections::BTreeMap,
    bitfield_struct::bitfield,
    core::mem,
    crate::asm,
    super::super::KIB,
};

const TABLE_SIZE: usize = 4 * KIB;
const PML4_TABLE_LENGTH: usize = TABLE_SIZE / mem::size_of::<Pml4e>();

/// # 4-Level Paging PML4E
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[derive(Debug)]
pub struct Pml4t<'a> {
    cr3: asm::control::Register3,
    vaddr2pdpt: BTreeMap<usize, Pdpt<'a>>,
}

impl From<asm::control::Register3> for Pml4t<'static> {
    fn from(cr3: asm::control::Register3) -> Self {
        let pml4t: u64 = cr3.get_page_directory_base();
        let pml4t: *mut [Pml4e; PML4_TABLE_LENGTH] = pml4t as *mut [Pml4e; PML4_TABLE_LENGTH];
        let pml4t: &mut [Pml4e; PML4_TABLE_LENGTH] = unsafe {
            &mut *pml4t
        };
        let vaddr2pdpt: BTreeMap<usize, Pdpt<'_>> = pml4t
            .iter_mut()
            .enumerate()
            .map(|(index, pml4e)| (index, pml4e.into()))
            .collect();
        Self {
            cr3,
            vaddr2pdpt,
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
    address_of_vaddr2pdpt: u64,
    #[bits(15, access = RO)]
    reserved1: u16,
    xd: bool,
}

/// # 4-Level Paging PML4E
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[derive(Debug)]
struct Pdpt<'a> {
    pml4e: &'a mut Pml4e,
}

impl<'a> From<&'a mut Pml4e> for Pdpt<'a> {
    fn from(pml4e: &'a mut Pml4e) -> Self {
        Self {
            pml4e,
        }
    }
}

