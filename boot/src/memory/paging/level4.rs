//! # 4-Level Paging
//! ## References
//! * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4.5 4-Level Paging and 5-Level Paging

use {
    alloc::{
        boxed::Box,
        collections::BTreeMap,
    },
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
    },
    crate::{
        memory,
        x64,
    },
};

const PML4T_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Pml4te>();
const PDPT_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Pdpte>();
const PDT_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Pdte>();
const PT_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Pte>();

/// # Page Map Level 4 Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(align(4096))]
struct Pml4t {
    pml4te: [Pml4te; PML4T_LENGTH],
}

/// # Page Map Level 4 Table Entry Interface
enum Pml4teInterface<'a> {
    Pml4e {
        pml4e: &'a Pml4e
    },
    Pml4teNotPresent {
        pml4te_not_present: &'a Pml4teNotPresent,
    },
}

impl<'a> Pml4teInterface<'a> {
    fn copy(source: &'a Pml4te, destination: &'a mut Pml4te) -> Self {
        match (source.pml4e(), source.pml4te_not_present()) {
            (Some(pml4e), None) => {
                let pml4e: &Pml4e = destination.set_pml4e(*pml4e);
                Self::Pml4e {
                    pml4e,
                }
            },
            (None, Some(pml4te_not_present)) => {
                let pml4te_not_present: &Pml4teNotPresent = destination.set_pml4te_not_present(*pml4te_not_present);
                Self::Pml4teNotPresent {
                    pml4te_not_present
                }
            },
            _ => panic!("Can't get a page map level 4 table entry."),
        }
    }
}

/// # Page Map Level 4 Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(C)]
union Pml4te {
    pml4e: Pml4e,
    pml4te_not_present: Pml4teNotPresent,
}

impl Pml4te {
    fn pml4e(&self) -> Option<&Pml4e> {
        let pml4e: &Pml4e = unsafe {
            &self.pml4e
        };
        if pml4e.p() {
            Some(pml4e)
        } else {
            None
        }
    }

    fn pml4te_not_present(&self) -> Option<&Pml4teNotPresent> {
        let pml4te_not_present: &Pml4teNotPresent = unsafe {
            &self.pml4te_not_present
        };
        if pml4te_not_present.p() {
            None
        } else {
            Some(pml4te_not_present)
        }
    }

    fn set_pml4e(&mut self, pml4e: Pml4e) -> &Pml4e {
        unsafe {
            self.pml4e = pml4e;
            &self.pml4e
        }
    }

    fn set_pml4te_not_present(&mut self, pml4te_not_present: Pml4teNotPresent) -> &Pml4teNotPresent {
        unsafe {
            self.pml4te_not_present = pml4te_not_present;
            &self.pml4te_not_present
        }
    }
}

/// # Page Map Level 4 Entry Present
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

/// # Page Map Level 4 Entry Not Present
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct Pml4teNotPresent {
    p: bool,
    #[bits(63, access = RO)]
    reserved0: u64,
}

/// # Page Directory Pointer Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(align(4096))]
struct Pdpt {
    pdpte: [Pdpte; PDPT_LENGTH],
}

impl<'a> From<&'a Pml4e> for &'a Pdpt {
    fn from(pml4e: &'a Pml4e) -> Self {
        let pdpt: u64 = pml4e.address_of_pdpt() << Pml4e::ADDRESS_OF_PDPT_OFFSET;
        let pdpt: *const Pdpt = pdpt as *const Pdpt;
        unsafe {
            &*pdpt
        }
    }
}

/// # Page Directory Pointer Table Entry Interface
enum PdpteInterface<'a> {
    Pe1Gib {
        pe1gib: &'a Pe1Gib,
    },
    Pdpe {
        pdpe: &'a Pdpe,
    },
    PdpteNotPresent {
        pdpte_not_present: &'a PdpteNotPresent,
    },
}

impl<'a> PdpteInterface<'a> {
    fn copy(source: &'a Pdpte, destination: &'a mut Pdpte) -> Self {
        match (source.pe1gib(), source.pdpe(), source.pdpte_not_present()) {
            (Some(pe1gib), None, None) => {
                let pe1gib: &Pe1Gib = destination.set_pe1gib(*pe1gib);
                Self::Pe1Gib {
                    pe1gib,
                }
            },
            (None, Some(pdpe), None) => {
                let pdpe: &Pdpe = destination.set_pdpe(*pdpe);
                Self::Pdpe {
                    pdpe,
                }
            },
            (None, None, Some(pdpte_not_present)) => {
                let pdpte_not_present: &PdpteNotPresent = destination.set_pdpte_not_present(*pdpte_not_present);
                Self::PdpteNotPresent {
                    pdpte_not_present,
                }
            },
            _ => panic!("Can't get a page directory pointer table entry."),
        }
    }
}

/// # Page Directory Pointer Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(C)]
union Pdpte {
    pe1gib: Pe1Gib,
    pdpe: Pdpe,
    pdpte_not_present: PdpteNotPresent,
}

impl Pdpte {
    fn pe1gib(&self) -> Option<&Pe1Gib> {
        let pe1gib: &Pe1Gib = unsafe {
            &self.pe1gib
        };
        if pe1gib.p() && pe1gib.page_1gib() {
            Some(pe1gib)
        } else {
            None
        }
    }

    fn pdpe(&self) -> Option<&Pdpe> {
        let pdpe: &Pdpe = unsafe {
            &self.pdpe
        };
        if pdpe.p() && !pdpe.page_1gib() {
            Some(pdpe)
        } else {
            None
        }
    }

    fn pdpte_not_present(&self) -> Option<&PdpteNotPresent> {
        let pdpte_not_present: &PdpteNotPresent = unsafe {
            &self.pdpte_not_present
        };
        if pdpte_not_present.p() {
            None
        } else {
            Some(pdpte_not_present)
        }
    }

    fn set_pe1gib(&mut self, pe1gib: Pe1Gib) -> &Pe1Gib {
        unsafe {
            self.pe1gib = pe1gib;
            &self.pe1gib
        }
    }

    fn set_pdpe(&mut self, pdpe: Pdpe) -> &Pdpe {
        unsafe {
            self.pdpe = pdpe;
            &self.pdpe
        }
    }

    fn set_pdpte_not_present(&mut self, pdpte_not_present: PdpteNotPresent) -> &PdpteNotPresent {
        unsafe {
            self.pdpte_not_present = pdpte_not_present;
            &self.pdpte_not_present
        }
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

/// # Page Directory Pointer Table Entry Not Present
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct PdpteNotPresent {
    p: bool,
    #[bits(63, access = RO)]
    reserved0: u64,
}

/// # Page Directory Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-22 Figure 4-8. Linear-Address Translation to a 4-KByte Page Using 4-Level Paging
#[repr(align(4096))]
struct Pdt {
    pdte: [Pdte; PDT_LENGTH],
}

impl<'a> From<&'a Pdpe> for &'a Pdt {
    fn from(pdpe: &'a Pdpe) -> Self {
        let pdt: u64 = pdpe.address_of_pdt() << Pdpe::ADDRESS_OF_PDT_OFFSET;
        let pdt: *const Pdt = pdt as *const Pdt;
        unsafe {
            &*pdt
        }
    }
}

/// # Page Directory Table Entry Interface
enum PdteInterface<'a> {
    Pe2Mib {
        pe2mib: &'a Pe2Mib,
    },
    Pde {
        pde: &'a Pde,
    },
    PdteNotPresent {
        pdte_not_present: &'a PdteNotPresent,
    },
}

impl<'a> PdteInterface<'a> {
    fn copy(source: &'a Pdte, destination: &'a mut Pdte) -> Self {
        match (source.pe2mib(), source.pde(), source.pdte_not_present()) {
            (Some(pe2mib), None, None) => {
                let pe2mib: &Pe2Mib = destination.set_pe2mib(*pe2mib);
                Self::Pe2Mib {
                    pe2mib,
                }
            },
            (None, Some(pde), None) => {
                let pde: &Pde = destination.set_pde(*pde);
                Self::Pde {
                    pde,
                }
            },
            (None, None, Some(pdte_not_present)) => {
                let pdte_not_present: &PdteNotPresent = destination.set_pdte_not_present(*pdte_not_present);
                Self::PdteNotPresent {
                    pdte_not_present,
                }
            },
            _ => panic!("Can't get a page directory table entry."),
        }
    }
}

/// # Page Directory Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(C)]
union Pdte {
    pe2mib: Pe2Mib,
    pde: Pde,
    pdte_not_present: PdteNotPresent,
}

impl Pdte {
    fn pe2mib(&self) -> Option<&Pe2Mib> {
        let pe2mib: &Pe2Mib = unsafe {
            &self.pe2mib
        };
        if pe2mib.p() && pe2mib.page_2mib() {
            Some(pe2mib)
        } else {
            None
        }
    }

    fn pde(&self) -> Option<&Pde> {
        let pde: &Pde = unsafe {
            &self.pde
        };
        if pde.p() && !pde.page_2mib() {
            Some(pde)
        } else {
            None
        }
    }

    fn pdte_not_present(&self) -> Option<&PdteNotPresent> {
        let pdte_not_present: &PdteNotPresent = unsafe {
            &self.pdte_not_present
        };
        if pdte_not_present.p() {
            None
        } else {
            Some(pdte_not_present)
        }
    }

    fn set_pe2mib(&mut self, pe2mib: Pe2Mib) -> &Pe2Mib {
        unsafe {
            self.pe2mib = pe2mib;
            &self.pe2mib
        }
    }

    fn set_pde(&mut self, pde: Pde) -> &Pde {
        unsafe {
            self.pde = pde;
            &self.pde
        }
    }

    fn set_pdte_not_present(&mut self, pdte_not_present: PdteNotPresent) -> &PdteNotPresent {
        unsafe {
            self.pdte_not_present = pdte_not_present;
            &self.pdte_not_present
        }
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

/// # Page Directory Table Entry Not Present
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct PdteNotPresent {
    p: bool,
    #[bits(63, access = RO)]
    reserved0: u64,
}

/// # Page Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-22 Figure 4-8. Linear-Address Translation to a 4-KByte Page Using 4-Level Paging
#[repr(align(4096))]
struct Pt {
    pte: [Pte; PT_LENGTH]
}

impl<'a> From<&'a Pde> for &'a Pt {
    fn from(pde: &'a Pde) -> Self {
        let pt: u64 = pde.address_of_pt() << Pde::ADDRESS_OF_PT_OFFSET;
        let pt: *const Pt = pt as *const Pt;
        unsafe {
            &*pt
        }
    }
}

/// # Page Table Entry Interface
enum PteInterface<'a> {
    Pe4Kib {
        pe4kib: &'a Pe4Kib,
    },
    PteNotPresent {
        pte_not_present: &'a PteNotPresent,
    },
}

impl<'a> PteInterface<'a> {
    fn copy(source: &'a Pte, destination: &'a mut Pte) -> Self {
        match (source.pe4kib(), source.pte_not_present()) {
            (Some(pe4kib), None) => {
                let pe4kib: &Pe4Kib = destination.set_pe4kib(*pe4kib);
                Self::Pe4Kib {
                    pe4kib,
                }
            },
            (None, Some(pte_not_present)) => {
                let pte_not_present: &PteNotPresent = destination.set_pte_not_present(*pte_not_present);
                Self::PteNotPresent {
                    pte_not_present,
                }
            },
            _ => panic!("Can't get a page table entry."),
        }
    }
}

/// # Page Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[derive(Clone, Copy)]
#[repr(C)]
union Pte {
    pe4kib: Pe4Kib,
    pte_not_present: PteNotPresent,
}

impl Pte {
    fn pe4kib(&self) -> Option<&Pe4Kib> {
        let pe4kib: &Pe4Kib = unsafe {
            &self.pe4kib
        };
        if pe4kib.p() {
            Some(pe4kib)
        } else {
            None
        }
    }

    fn pte_not_present(&self) -> Option<&PteNotPresent> {
        let pte_not_present: &PteNotPresent = unsafe {
            &self.pte_not_present
        };
        if pte_not_present.p() {
            None
        } else {
            Some(pte_not_present)
        }
    }

    fn set_pe4kib(&mut self, pe4kib: Pe4Kib) -> &Pe4Kib {
        unsafe {
            self.pe4kib = pe4kib;
            &self.pe4kib
        }
    }

    fn set_pte_not_present(&mut self, pte_not_present: PteNotPresent) -> &PteNotPresent {
        unsafe {
            self.pte_not_present = pte_not_present;
            &self.pte_not_present
        }
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

/// # Page Table Entry Not Present
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[bitfield(u64)]
struct PteNotPresent {
    p: bool,
    #[bits(63, access = RO)]
    reserved0: u64,
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

