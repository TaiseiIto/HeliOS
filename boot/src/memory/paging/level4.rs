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
    crate::x64,
    super::super::KIB,
};

const TABLE_SIZE: usize = 4 * KIB;
const PML4T_LENGTH: usize = TABLE_SIZE / mem::size_of::<Pml4e>();
const PDPT_LENGTH: usize = TABLE_SIZE / mem::size_of::<Pdpe>();
const PDT_LENGTH: usize = TABLE_SIZE / mem::size_of::<Pde>();
const PT_LENGTH: usize = TABLE_SIZE / mem::size_of::<Pe4Kib>();

/// # Page Map Level 4 Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
pub struct Pml4t<'a> {
    cr3: x64::control::Register3,
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

impl From<x64::control::Register3> for Pml4t<'static> {
    fn from(cr3: x64::control::Register3) -> Self {
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
    vaddr2pdt_or_pe1gib: Option<BTreeMap<usize, PdtOrPe1Gib<'a>>>,
}

impl<'a> Pdpt<'a> {
    fn exists(&self) -> bool {
        self.pml4e.p()
    }

    fn new(pml4i: usize, pml4e: &'a mut Pml4e) -> Self {
        let vaddr2pdt_or_pe1gib: Option<BTreeMap<usize, PdtOrPe1Gib<'a>>> = if pml4e.p() {
            let pdpt: usize = pml4e.get_pdpt();
            let pdpt: *mut [u64; PDPT_LENGTH] = pdpt as *mut [u64; PDPT_LENGTH];
            let pdpt: &mut [u64; PDPT_LENGTH] = unsafe {
                &mut *pdpt
            };
            Some(pdpt
                .iter_mut()
                .enumerate()
                .map(|(pdpi, pdpe)| (Vaddr::create(pml4i, pdpi, 0, 0, 0).into(), PdtOrPe1Gib::new(pml4i, pdpi, pdpe)))
                .collect())
        } else {
            None
        };
        Self {
            pml4e,
            vaddr2pdt_or_pe1gib,
        }
    }
}

impl fmt::Debug for Pdpt<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = formatter.debug_struct("Pdpt");
        debug_struct.field("pml4e", &self.pml4e);
        if let Some(vaddr2pdt_or_pe1gib) = self.vaddr2pdt_or_pe1gib.as_ref() {
            let vaddr2pdt_or_pe1gib: BTreeMap<&usize, &PdtOrPe1Gib<'_>> = vaddr2pdt_or_pe1gib
                .iter()
                .filter(|(_vaddr, pdt_or_pe1gib)| pdt_or_pe1gib.exists())
                .collect();
            debug_struct.field("vaddr2pdt_or_pe1gib", &vaddr2pdt_or_pe1gib);
        }
        debug_struct.finish()
    }
}

/// # Page Directory Table or 1GiB Page Entry
#[derive(Debug)]
enum PdtOrPe1Gib<'a> {
    Pdt(Pdt<'a>),
    Pe1Gib(&'a mut Pe1Gib),
}

impl<'a> PdtOrPe1Gib<'a> {
    fn exists(&self) -> bool {
        match self {
            Self::Pdt(pdt) => pdt.exists(),
            Self::Pe1Gib(pe1gib) => pe1gib.p(),
        }
    }

    fn new(pml4i: usize, pdpi: usize, pdpe: &'a mut u64) -> Self {
        match *pdpe & 1 << Pdpe::PAGE_1GIB_OFFSET {
            0 => {
                let pdpe: *mut u64 = pdpe as *mut u64;
                let pdpe: *mut Pdpe = pdpe as *mut Pdpe;
                Self::Pdt(Pdt::new(pml4i, pdpi, unsafe {
                    &mut *pdpe
                }))
            },
            _ => {
                let pe1gib: *mut u64 = pdpe as *mut u64;
                let pe1gib: *mut Pe1Gib = pe1gib as *mut Pe1Gib;
                Self::Pe1Gib(unsafe {
                    &mut *pe1gib
                })
            },
        }
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

impl Pdpe {
    fn get_pdt(&self) -> usize {
        (self.address_of_pdt() << Self::ADDRESS_OF_PDT_OFFSET) as usize
    }
}

/// # 1GiB Page Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct Pe1Gib {
    p: bool,
    rw: bool,
    us: bool,
    pwt: bool,
    pcd: bool,
    a: bool,
    d: bool,
    page_1gib: bool,
    g: bool,
    #[bits(2, access = RO)]
    reserved0: u8,
    r: bool,
    pat: bool,
    #[bits(17, access = RO)]
    reserved1: u32,
    #[bits(18)]
    address_of_1gib_page_frame: u32,
    #[bits(11, access = RO)]
    reserved2: u16,
    #[bits(4)]
    prot_key: u8,
    xd: bool,
}

/// # Page Directory Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-22 Figure 4-8. Linear-Address Translation to a 4-KByte Page Using 4-Level Paging
struct Pdt<'a> {
    pdpe: &'a mut Pdpe,
    vaddr2pt_or_pe2mib: Option<BTreeMap<usize, PtOrPe2Mib<'a>>>,
}

impl<'a> Pdt<'a> {
    fn exists(&self) -> bool {
        self.pdpe.p()
    }

    fn new(pml4i: usize, pdpi: usize, pdpe: &'a mut Pdpe) -> Self {
        let vaddr2pt_or_pe2mib: Option<BTreeMap<usize, PtOrPe2Mib<'a>>> = if pdpe.p() {
            let pdt: usize = pdpe.get_pdt();
            let pdt: *mut [u64; PDT_LENGTH] = pdt as *mut [u64; PDT_LENGTH];
            let pdt: &mut [u64; PDT_LENGTH] = unsafe {
                &mut *pdt
            };
            Some(pdt
                .iter_mut()
                .enumerate()
                .map(|(pdi, pde)| (Vaddr::create(pml4i, pdpi, pdi, 0, 0).into(), PtOrPe2Mib::new(pml4i, pdpi, pdi, pde)))
                .collect())
        } else {
            None
        };
        Self {
            pdpe,
            vaddr2pt_or_pe2mib,
        }
    }
}

impl fmt::Debug for Pdt<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = formatter.debug_struct("Pdt");
        debug_struct.field("pdpe", &self.pdpe);
        if let Some(vaddr2pt_or_pe2mib) = self.vaddr2pt_or_pe2mib.as_ref() {
            let vaddr2pt_or_pe2mib: BTreeMap<&usize, &PtOrPe2Mib<'_>> = vaddr2pt_or_pe2mib
                .iter()
                .filter(|(_vaddr, pt_or_pe2mib)| pt_or_pe2mib.exists())
                .collect();
            debug_struct.field("vaddr2pt_or_pe2mib", &vaddr2pt_or_pe2mib);
        }
        debug_struct.finish()
    }
}

/// # Page Table or 2MiB Page Entry
#[derive(Debug)]
enum PtOrPe2Mib<'a> {
    Pt(Pt<'a>),
    Pe2Mib(&'a mut Pe2Mib),
}

impl<'a> PtOrPe2Mib<'a> {
    fn exists(&self) -> bool {
        match self {
            Self::Pt(pt) => pt.exists(),
            Self::Pe2Mib(pe2mib) => pe2mib.p(),
        }
    }

    fn new(pml4i: usize, pdpi: usize, pdi: usize, pde: &'a mut u64) -> Self {
        match *pde & 1 << Pde::PAGE_2MIB_OFFSET {
            0 => {
                let pde: *mut u64 = pde as *mut u64;
                let pde: *mut Pde = pde as *mut Pde;
                Self::Pt(Pt::new(pml4i, pdpi, pdi, unsafe {
                    &mut *pde
                }))
            },
            _ => {
                let pe2mib: *mut u64 = pde as *mut u64;
                let pe2mib: *mut Pe2Mib = pe2mib as *mut Pe2Mib;
                Self::Pe2Mib(unsafe {
                    &mut *pe2mib
                })
            },
        }
    }
}

/// # Page Directory Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct Pde {
    p: bool,
    rw: bool,
    us: bool,
    pwt: bool,
    pcd: bool,
    a: bool,
    #[bits(access = RO)]
    reserved0: bool,
    page_2mib: bool,
    #[bits(3, access = RO)]
    reserved1: u8,
    r: bool,
    #[bits(36)]
    address_of_pt: u64,
    #[bits(15, access = RO)]
    reserved2: u16,
    xd: bool,
}

impl Pde {
    fn get_pt(&self) -> usize {
        (self.address_of_pt() << Self::ADDRESS_OF_PT_OFFSET) as usize
    }
}

/// # 2MiB Page Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct Pe2Mib {
    p: bool,
    rw: bool,
    us: bool,
    pwt: bool,
    pcd: bool,
    a: bool,
    d: bool,
    page_2mib: bool,
    g: bool,
    #[bits(2, access = RO)]
    reserved0: u8,
    r: bool,
    pat: bool,
    #[bits(8, access = RO)]
    reserved1: u32,
    #[bits(27)]
    address_of_2mib_page_frame: u32,
    #[bits(11, access = RO)]
    reserved2: u16,
    #[bits(4)]
    prot_key: u8,
    xd: bool,
}

/// # Page Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-22 Figure 4-8. Linear-Address Translation to a 4-KByte Page Using 4-Level Paging
struct Pt<'a> {
    pde: &'a mut Pde,
    vaddr2pe4kib: Option<BTreeMap<usize, &'a mut Pe4Kib>>
}

impl<'a> Pt<'a> {
    fn exists(&self) -> bool {
        self.pde.p()
    }

    fn new(pml4i: usize, pdpi: usize, pdi: usize, pde: &'a mut Pde) -> Self {
        let vaddr2pe4kib: Option<BTreeMap<usize, &'a mut Pe4Kib>> = if pde.p() && !pde.page_2mib() {
            let pt: usize = pde.get_pt();
            let pt: *mut [Pe4Kib; PT_LENGTH] = pt as *mut [Pe4Kib; PT_LENGTH];
            let pt: &mut [Pe4Kib; PT_LENGTH] = unsafe {
                &mut *pt
            };
            Some(pt
                .iter_mut()
                .enumerate()
                .map(|(pi, pe)| (Vaddr::create(pml4i, pdpi, pdi, pi, 0).into(), pe))
                .collect())
        } else {
            None
        };
        Self {
            pde,
            vaddr2pe4kib,
        }
    }
}

impl fmt::Debug for Pt<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = formatter.debug_struct("Pt");
        debug_struct.field("pde", &self.pde);
        if let Some(vaddr2pe4kib) = self.vaddr2pe4kib.as_ref() {
            let vaddr2pe4kib: BTreeMap<&usize, &&mut Pe4Kib> = vaddr2pe4kib
                .iter()
                .filter(|(_vaddr, pe4kib)| pe4kib.p())
                .collect();
            debug_struct.field("vaddr2pe4kib", &vaddr2pe4kib);
        }
        debug_struct.finish()
    }
}

/// # 4KiB Page Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct Pe4Kib {
    p: bool,
    rw: bool,
    us: bool,
    pwt: bool,
    pcd: bool,
    a: bool,
    d: bool,
    pat: bool,
    g: bool,
    #[bits(2, access = RO)]
    reserved0: u8,
    r: bool,
    #[bits(36)]
    address_of_4mib_page_frame: u64,
    #[bits(11, access = RO)]
    reserved1: u16,
    #[bits(4)]
    prot_key: u8,
    xd: bool,
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

