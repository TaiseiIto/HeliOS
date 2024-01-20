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

pub struct Interface {
    cr3: x64::control::Register3,
    pml4t: Box<Pml4t>,
    vaddr2pml4te_interface: BTreeMap<Vaddr, Pml4teInterface>,
}

impl Interface {
    pub fn get(cr3: x64::control::Register3) -> Self {
        let source: &Pml4t = cr3.get_paging_structure();
        let mut pml4t: Box<Pml4t> = Box::new(Pml4t::default());
        let cr3: x64::control::Register3 = cr3.with_paging_structure(pml4t.as_ref());
        let vaddr2pml4te_interface: BTreeMap<Vaddr, Pml4teInterface> = source.pml4te
            .as_slice()
            .iter()
            .zip(pml4t
                .as_mut()
                .pml4te
                .as_mut_slice()
                .iter_mut())
            .enumerate()
            .map(|(pml4i, (source, destination))| {
                let vaddr = Vaddr::create(pml4i, 0, 0, 0, 0);
                (vaddr, Pml4teInterface::copy(source, destination, vaddr))
            })
            .collect();
        Self {
            cr3,
            pml4t,
            vaddr2pml4te_interface,
        }
    }
}

/// # Page Map Level 4 Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(align(4096))]
struct Pml4t {
    pml4te: [Pml4te; PML4T_LENGTH],
}

impl Default for Pml4t {
    fn default() -> Self {
        let pml4te = [Pml4te::default(); PML4T_LENGTH];
        Self {
            pml4te,
        }
    }
}

impl<'a> From<&'a x64::control::Register3> for &'a Pml4t {
    fn from(cr3: &'a x64::control::Register3) -> Self {
        cr3.get_paging_structure()
    }
}

/// # Page Map Level 4 Table Entry Interface
enum Pml4teInterface {
    Pml4e {
        pdpt: Box<Pdpt>,
        vaddr2pdpte_interface: BTreeMap<Vaddr, PdpteInterface>,
    },
    Pml4teNotPresent,
}

impl Pml4teInterface {
    fn copy(source: &Pml4te, destination: &mut Pml4te, vaddr: Vaddr) -> Self {
        match (source.pml4e(), source.pml4te_not_present()) {
            (Some(pml4e), None) => {
                let source: &Pdpt = pml4e.into();
                let mut pdpt: Box<Pdpt> = Box::new(Pdpt::default());
                destination.set_pml4e(*pml4e, pdpt.as_ref());
                let vaddr2pdpte_interface: BTreeMap<Vaddr, PdpteInterface> = source.pdpte
                    .as_slice()
                    .iter()
                    .zip(pdpt
                        .as_mut()
                        .pdpte
                        .as_mut_slice()
                        .iter_mut())
                    .enumerate()
                    .map(|(pdpi, (source, destination))| {
                        let vaddr: Vaddr = vaddr.with_pdpi(pdpi as u16);
                        (vaddr, PdpteInterface::copy(source, destination, vaddr))
                    })
                    .collect();
                Self::Pml4e {
                    pdpt,
                    vaddr2pdpte_interface,
                }
            },
            (None, Some(pml4te_not_present)) => {
                destination.set_pml4te_not_present(*pml4te_not_present);
                Self::Pml4teNotPresent
            },
            _ => panic!("Can't get a page map level 4 table entry."),
        }
    }
}

/// # Page Map Level 4 Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[derive(Clone, Copy)]
#[repr(C)]
union Pml4te {
    pml4e: Pml4e,
    pml4te_not_present: Pml4teNotPresent,
}

impl Default for Pml4te {
    fn default() -> Self {
        let pml4te_not_present = Pml4teNotPresent::default();
        Self {
            pml4te_not_present
        }
    }
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

    fn set_pml4e(&mut self, pml4e: Pml4e, pdpt: &Pdpt) {
        let pdpt: *const Pdpt = pdpt as *const Pdpt;
        let pdpt: u64 = pdpt as u64;
        let pdpt: u64 = pdpt >> Pml4e::ADDRESS_OF_PDPT_OFFSET;
        unsafe {
            self.pml4e = pml4e;
            self.pml4e.set_address_of_pdpt(pdpt);
        }
        assert!(self.pml4e().is_some());
        assert!(self.pml4te_not_present().is_none());
    }

    fn set_pml4te_not_present(&mut self, pml4te_not_present: Pml4teNotPresent) {
        unsafe {
            self.pml4te_not_present = pml4te_not_present;
        }
        assert!(self.pml4e().is_none());
        assert!(self.pml4te_not_present().is_some());
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

impl Default for Pdpt {
    fn default() -> Self {
        let pdpte = [Pdpte::default(); PDPT_LENGTH];
        Self {
            pdpte,
        }
    }
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
enum PdpteInterface {
    Pe1Gib,
    Pdpe {
        pdt: Box<Pdt>,
        vaddr2pdte_interface: BTreeMap<Vaddr, PdteInterface>,
    },
    PdpteNotPresent,
}

impl PdpteInterface {
    fn copy(source: &Pdpte, destination: &mut Pdpte, vaddr: Vaddr) -> Self {
        match (source.pe1gib(), source.pdpe(), source.pdpte_not_present()) {
            (Some(pe1gib), None, None) => {
                destination.set_pe1gib(*pe1gib);
                Self::Pe1Gib
            },
            (None, Some(pdpe), None) => {
                let source: &Pdt = pdpe.into();
                let mut pdt: Box<Pdt> = Box::new(Pdt::default());
                destination.set_pdpe(*pdpe, pdt.as_ref());
                let vaddr2pdte_interface: BTreeMap<Vaddr, PdteInterface> = source.pdte
                    .as_slice()
                    .iter()
                    .zip(pdt
                        .as_mut()
                        .pdte
                        .as_mut_slice()
                        .iter_mut())
                    .enumerate()
                    .map(|(pdi, (source, destination))| {
                        let vaddr: Vaddr = vaddr.with_pdi(pdi as u16);
                        (vaddr, PdteInterface::copy(source, destination, vaddr))
                    })
                    .collect();
                Self::Pdpe {
                    pdt,
                    vaddr2pdte_interface,
                }
            },
            (None, None, Some(pdpte_not_present)) => {
                destination.set_pdpte_not_present(*pdpte_not_present);
                Self::PdpteNotPresent
            },
            _ => panic!("Can't get a page directory pointer table entry."),
        }
    }
}

/// # Page Directory Pointer Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[derive(Clone, Copy)]
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

    fn set_pe1gib(&mut self, pe1gib: Pe1Gib) {
        unsafe {
            self.pe1gib = pe1gib;
        }
        assert!(self.pe1gib().is_some());
        assert!(self.pdpe().is_none());
        assert!(self.pdpte_not_present().is_none());
    }

    fn set_pdpe(&mut self, pdpe: Pdpe, pdt: &Pdt) {
        let pdt: *const Pdt = pdt as *const Pdt;
        let pdt: u64 = pdt as u64;
        let pdt: u64 = pdt >> Pdpe::ADDRESS_OF_PDT_OFFSET;
        unsafe {
            self.pdpe = pdpe;
            self.pdpe.set_address_of_pdt(pdt);
        }
        assert!(self.pe1gib().is_none());
        assert!(self.pdpe().is_some());
        assert!(self.pdpte_not_present().is_none());
    }

    fn set_pdpte_not_present(&mut self, pdpte_not_present: PdpteNotPresent) {
        unsafe {
            self.pdpte_not_present = pdpte_not_present;
        }
        assert!(self.pe1gib().is_none());
        assert!(self.pdpe().is_none());
        assert!(self.pdpte_not_present().is_none());
    }
}

impl Default for Pdpte {
    fn default() -> Self {
        let pdpte_not_present = PdpteNotPresent::default();
        Self {
            pdpte_not_present
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

impl Default for Pdt {
    fn default() -> Self {
        let pdte = [Pdte::default(); PDT_LENGTH];
        Self {
            pdte,
        }
    }
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
enum PdteInterface {
    Pe2Mib,
    Pde {
        pt: Box<Pt>,
        vaddr2pte_interface: BTreeMap<Vaddr, PteInterface>,
    },
    PdteNotPresent,
}

impl PdteInterface {
    fn copy(source: &Pdte, destination: &mut Pdte, vaddr: Vaddr) -> Self {
        match (source.pe2mib(), source.pde(), source.pdte_not_present()) {
            (Some(pe2mib), None, None) => {
                destination.set_pe2mib(*pe2mib);
                Self::Pe2Mib
            },
            (None, Some(pde), None) => {
                let source: &Pt = pde.into();
                let mut pt: Box<Pt> = Box::new(Pt::default());
                destination.set_pde(*pde, pt.as_ref());
                let vaddr2pte_interface: BTreeMap<Vaddr, PteInterface> = source.pte
                    .as_slice()
                    .iter()
                    .zip(pt
                        .as_mut()
                        .pte
                        .as_mut_slice()
                        .iter_mut())
                    .enumerate()
                    .map(|(pi, (source, destination))| (vaddr.with_pi(pi as u16), PteInterface::copy(source, destination)))
                    .collect();
                Self::Pde {
                    pt,
                    vaddr2pte_interface,
                }
            },
            (None, None, Some(pdte_not_present)) => {
                destination.set_pdte_not_present(*pdte_not_present);
                Self::PdteNotPresent
            },
            _ => panic!("Can't get a page directory table entry."),
        }
    }
}

/// # Page Directory Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[derive(Clone, Copy)]
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

    fn set_pe2mib(&mut self, pe2mib: Pe2Mib) {
        unsafe {
            self.pe2mib = pe2mib;
        }
        assert!(self.pe2mib().is_some());
        assert!(self.pde().is_none());
        assert!(self.pdte_not_present().is_none());
    }

    fn set_pde(&mut self, pde: Pde, pt: &Pt) {
        let pt: *const Pt = pt as *const Pt;
        let pt: u64 = pt as u64;
        let pt: u64 = pt >> Pde::ADDRESS_OF_PT_OFFSET;
        unsafe {
            self.pde = pde;
            self.pde.set_address_of_pt(pt);
        }
        assert!(self.pe2mib().is_none());
        assert!(self.pde().is_some());
        assert!(self.pdte_not_present().is_none());
    }

    fn set_pdte_not_present(&mut self, pdte_not_present: PdteNotPresent) {
        unsafe {
            self.pdte_not_present = pdte_not_present;
        }
        assert!(self.pe2mib().is_none());
        assert!(self.pde().is_none());
        assert!(self.pdte_not_present().is_some());
    }
}

impl Default for Pdte {
    fn default() -> Self {
        let pdte_not_present = PdteNotPresent::default();
        Self {
            pdte_not_present
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

impl Default for Pt {
    fn default() -> Self {
        let pte = [Pte::default(); PT_LENGTH];
        Self {
            pte,
        }
    }
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
enum PteInterface {
    Pe4Kib,
    PteNotPresent,
}

impl PteInterface {
    fn copy(source: &Pte, destination: &mut Pte) -> Self {
        match (source.pe4kib(), source.pte_not_present()) {
            (Some(pe4kib), None) => {
                destination.set_pe4kib(*pe4kib);
                Self::Pe4Kib
            },
            (None, Some(pte_not_present)) => {
                destination.set_pte_not_present(*pte_not_present);
                Self::PteNotPresent
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

    fn set_pe4kib(&mut self, pe4kib: Pe4Kib) {
        unsafe {
            self.pe4kib = pe4kib;
        }
        assert!(self.pe4kib().is_some());
        assert!(self.pte_not_present().is_none());
    }

    fn set_pte_not_present(&mut self, pte_not_present: PteNotPresent) {
        unsafe {
            self.pte_not_present = pte_not_present;
        }
        assert!(self.pe4kib().is_none());
        assert!(self.pte_not_present().is_some());
    }
}

impl Default for Pte {
    fn default() -> Self {
        let pte_not_present = PteNotPresent::default();
        Self {
            pte_not_present
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
#[derive(Eq, Ord, PartialEq, PartialOrd)]
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

