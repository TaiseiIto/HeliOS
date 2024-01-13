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
            .map(|(index, pml4e)| (Vaddr::create(index, 0, 0, 0, 0).into(), pml4e.into()))
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

/// # 4-Level Paging PML4E
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-22 Figure 4-8. Linear-Address Translation to a 4-KByte Page Using 4-Level Paging
#[bitfield(u64)]
struct Vaddr {
    #[bits(12)]
    offset: u16,
    #[bits(9)]
    p: u16,
    #[bits(9)]
    pd: u16,
    #[bits(9)]
    pdp: u16,
    #[bits(9)]
    pml4: u16,
    reserved0: u16,
}

impl Vaddr {
    fn create(pml4: usize, pdp: usize, pd: usize, p: usize, offset: usize) -> Self {
        let pml4: u16 = pml4 as u16;
        let pdp: u16 = pdp as u16;
        let pd: u16 = pd as u16;
        let p: u16 = p as u16;
        let offset: u16 = offset as u16;
        Self::new()
            .with_offset(offset)
            .with_p(p)
            .with_pd(pd)
            .with_pdp(pdp)
            .with_pml4(pml4)
            .with_reserved0(match pml4 & (1 << (Self::PML4_BITS - 1)) {
                0 => 0,
                _ => 0xffff,
            })
    }
}

impl From<Vaddr> for usize {
    fn from(vaddr: Vaddr) -> Self {
        let vaddr: u64 = vaddr.into();
        vaddr as Self
    }
}

