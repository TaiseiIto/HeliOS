//! # 4-Level Paging
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4.5 4-Level Paging and 5-Level Paging

use {
    alloc::collections::BTreeMap,
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
    },
    crate::asm,
    super::super::KIB,
};

const TABLE_SIZE: usize = 4 * KIB;
const PML4T_LENGTH: usize = TABLE_SIZE / mem::size_of::<Pml4e>();
const PDPT_LENGTH: usize = TABLE_SIZE / mem::size_of::<Pdpe>();

/// # Page Map Level 4 Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
pub struct Pml4t<'a> {
    cr3: asm::control::Register3,
    vaddr2pdpt: BTreeMap<usize, Pdpt<'a>>,
}

impl fmt::Debug for Pml4t<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vaddr2pdpt: BTreeMap<&usize, &Pdpt<'_>> = self.vaddr2pdpt
            .iter()
            .filter(|(_vaddr, pdpt)| pdpt.exists())
            .collect();
        formatter
            .debug_struct("Pml4t")
            .field("cr3", &self.cr3)
            .field("vaddr2pdpt", &vaddr2pdpt)
            .finish()
    }
}

impl From<asm::control::Register3> for Pml4t<'static> {
    fn from(cr3: asm::control::Register3) -> Self {
        let pml4t: usize = cr3.get_page_directory_base();
        let pml4t: *mut [Pml4e; PML4T_LENGTH] = pml4t as *mut [Pml4e; PML4T_LENGTH];
        let pml4t: &mut [Pml4e; PML4T_LENGTH] = unsafe {
            &mut *pml4t
        };
        let vaddr2pdpt: BTreeMap<usize, Pdpt<'_>> = pml4t
            .iter_mut()
            .enumerate()
            .map(|(pml4i, pml4e)| (Vaddr::create(pml4i, 0, 0, 0, 0).into(), Pdpt::new(pml4i, pml4e)))
            .collect();
        Self {
            cr3,
            vaddr2pdpt,
        }
    }
}

/// # Page Map Level 4 Entry
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
    address_of_pdpt: u64,
    #[bits(15, access = RO)]
    reserved1: u16,
    xd: bool,
}

impl Pml4e {
    fn get_pdpt(&self) -> usize {
        (self.address_of_pdpt() << Self::ADDRESS_OF_PDPT_OFFSET) as usize
    }
}

/// # Page Directory Pointer Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
struct Pdpt<'a> {
    pml4e: &'a mut Pml4e,
    vaddr2pdt: Option<BTreeMap<usize, Pdt<'a>>>,
}

impl<'a> Pdpt<'a> {
    fn exists(&self) -> bool {
        self.pml4e.p()
    }

    fn new(pml4i: usize, pml4e: &'a mut Pml4e) -> Self {
        let pdpt: usize = pml4e.get_pdpt();
        let pdpt: *mut [Pdpe; PDPT_LENGTH] = pdpt as *mut [Pdpe; PDPT_LENGTH];
        let pdpt: &mut [Pdpe; PDPT_LENGTH] = unsafe {
            &mut *pdpt
        };
        let vaddr2pdt: Option<BTreeMap<usize, Pdt<'_>>> = if pml4e.p() {
            Some(pdpt
                .iter_mut()
                .enumerate()
                .map(|(pdpi, pdpe)| (Vaddr::create(pml4i, pdpi, 0, 0, 0).into(), Pdt::new(pdpe)))
                .collect())
        } else {
            None
        };
        Self {
            pml4e,
            vaddr2pdt,
        }
    }
}

impl fmt::Debug for Pdpt<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = formatter.debug_struct("Pdpt");
        debug_struct.field("pml4e", &self.pml4e);
        if let Some(vaddr2pdt) = self.vaddr2pdt.as_ref() {
            let vaddr2pdt: BTreeMap<&usize, &Pdt<'_>> = vaddr2pdt
                .iter()
                .filter(|(_vaddr, pdt)| pdt.exists())
                .collect();
            debug_struct.field("vaddr2pdt", &vaddr2pdt);
        }
        debug_struct.finish()
    }
}

/// # Page Directory Pointer Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct Pdpe {
    p: bool,
    rw: bool,
    us: bool,
    pwt: bool,
    pcd: bool,
    a: bool,
    #[bits(access = RO)]
    reserved0: bool,
    page_1gib: bool,
    #[bits(3, access = RO)]
    reserved1: u8,
    r: bool,
    #[bits(36)]
    address_of_pdt: u64,
    #[bits(15, access = RO)]
    reserved2: u16,
    xd: bool,
}

/// # Page Directory Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-22 Figure 4-8. Linear-Address Translation to a 4-KByte Page Using 4-Level Paging
#[derive(Debug)]
struct Pdt<'a> {
    pdpe: &'a mut Pdpe,
}

impl<'a> Pdt<'a> {
    fn exists(&self) -> bool {
        self.pdpe.p()
    }

    fn new(pdpe: &'a mut Pdpe) -> Self {
        Self {
            pdpe,
        }
    }
}

/// # Virtual Address
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-22 Figure 4-8. Linear-Address Translation to a 4-KByte Page Using 4-Level Paging
/// ## Fields
/// * `offset` - Offset from page base.
/// * `pi` - Page index.
/// * `pdi` - Page directory index.
/// * `pdpi` - Page directory pointer index.
/// * `pml4i` - Page map level 4 index.
#[bitfield(u64)]
struct Vaddr {
    #[bits(12)]
    offset: u16,
    #[bits(9)]
    pi: u16,
    #[bits(9)]
    pdi: u16,
    #[bits(9)]
    pdpi: u16,
    #[bits(9)]
    pml4i: u16,
    reserved0: u16,
}

impl Vaddr {
    fn create(pml4i: usize, pdpi: usize, pdi: usize, pi: usize, offset: usize) -> Self {
        let pml4i: u16 = pml4i as u16;
        let pdpi: u16 = pdpi as u16;
        let pdi: u16 = pdi as u16;
        let pi: u16 = pi as u16;
        let offset: u16 = offset as u16;
        Self::new()
            .with_offset(offset)
            .with_pi(pi)
            .with_pdi(pdi)
            .with_pdpi(pdpi)
            .with_pml4i(pml4i)
            .with_reserved0(match pml4i & (1 << (Self::PML4I_BITS - 1)) {
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

