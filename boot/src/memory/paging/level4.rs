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
        ops::Range,
    },
    crate::{
        com2_print,
        com2_println,
        memory,
        x64,
    },
};

const PML4T_LENGTH: usize = memory::page::SIZE / mem::size_of::<Pml4te>();
const PDPT_LENGTH: usize = memory::page::SIZE / mem::size_of::<Pdpte>();
const PDT_LENGTH: usize = memory::page::SIZE / mem::size_of::<Pdte>();
const PT_LENGTH: usize = memory::page::SIZE / mem::size_of::<Pte>();

pub struct Controller {
    cr3: x64::control::Register3,
    pml4t: Box<Pml4t>,
    vaddr2pml4te_controller: BTreeMap<Vaddr, Pml4teController>,
}

impl Controller {
    #[allow(dead_code)]
    pub fn debug(&self, vaddr: usize) {
        com2_println!("cr3 = {:#x?}", self.cr3);
        let vaddr: Vaddr = vaddr.into();
        let pml4vaddr: Vaddr = vaddr
                .with_pdpi(0)
                .with_pdi(0)
                .with_pi(0)
                .with_offset(0);
        let pml4te: &Pml4te = self.pml4t
            .as_ref()
            .pml4te(&pml4vaddr);
        com2_println!("pml4te = {:#x?}", pml4te);
        if let Some(pml4te_controller) = self.vaddr2pml4te_controller.get(&pml4vaddr) {
            pml4te_controller.debug(&vaddr);
        }
    }

    pub fn get(cr3: x64::control::Register3) -> Self {
        let source: &Pml4t = cr3.get_paging_structure();
        let mut pml4t = Box::<Pml4t>::new(source.clone());
        let cr3: x64::control::Register3 = cr3.with_paging_structure(pml4t.as_ref());
        let vaddr2pml4te_controller = BTreeMap::<Vaddr, Pml4teController>::new();
        Self {
            cr3,
            pml4t,
            vaddr2pml4te_controller,
        }
    }

    pub fn higher_half_range(&self) -> Range<u128> {
        let start_pml4i: usize = 1 << (Vaddr::PML4I_BITS - 1);
        let start_pdpi: usize = 0;
        let start_pdi: usize = 0;
        let start_pi: usize = 0;
        let start_offset: usize = 0;
        let start = Vaddr::create(start_pml4i, start_pdpi, start_pdi, start_pi, start_offset);
        let start: usize = start.into();
        let start: u128 = start as u128;
        let end_pml4i: usize = (1 << Vaddr::PML4I_BITS) - 1;
        let end_pdpi: usize = (1 << Vaddr::PDPI_BITS) - 1;
        let end_pdi: usize = (1 << Vaddr::PDI_BITS) - 1;
        let end_pi: usize = (1 << Vaddr::PI_BITS) - 1;
        let end_offset: usize = (1 << Vaddr::OFFSET_BITS) - 1;
        let end = Vaddr::create(end_pml4i, end_pdpi, end_pdi, end_pi, end_offset);
        let end: usize = end.into();
        let end: u128 = end as u128 + 1;
        start..end
    }

    pub fn set(&self) {
        self.cr3.set()
    }

    pub fn set_page(&mut self, vaddr: usize, paddr: usize, present: bool, writable: bool, executable: bool) {
        let vaddr: Vaddr = vaddr.into();
        let pml4vaddr: Vaddr = vaddr
                .with_pdpi(0)
                .with_pdi(0)
                .with_pi(0)
                .with_offset(0);
        if !self.vaddr2pml4te_controller.contains_key(&pml4vaddr) {
            let pml4i: usize = vaddr.pml4i() as usize;
            let pml4te_controller: Pml4teController = self
                .pml4t
                .pml4te
                .as_slice()
                .get(pml4i)
                .unwrap()
                .into();
        }
        let pml4te: &mut Pml4te = self.pml4t
            .as_mut()
            .pml4te_mut(&pml4vaddr);
        self.vaddr2pml4te_controller
            .get_mut(&pml4vaddr)
            .unwrap()
            .set_page(pml4te, &vaddr, paddr, present, writable, executable);
    }
}

impl fmt::Debug for Controller {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries(self.vaddr2pml4te_controller
                .iter()
                .zip(self.pml4t
                    .as_ref()
                    .pml4te
                    .as_slice()
                    .iter())
                .map(|((vaddr, pml4te_controller), pml4te)| (vaddr, (pml4te, pml4te_controller))))
            .finish()
    }
}

/// # Page Map Level 4 Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[derive(Clone)]
#[repr(align(4096))]
struct Pml4t {
    pml4te: [Pml4te; PML4T_LENGTH],
}

impl Pml4t {
    #[allow(dead_code)]
    fn pml4te(&self, vaddr: &Vaddr) -> &Pml4te {
        &self.pml4te[vaddr.pml4i() as usize]
    }

    fn pml4te_mut(&mut self, vaddr: &Vaddr) -> &mut Pml4te {
        &mut self.pml4te[vaddr.pml4i() as usize]
    }
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

/// # Page Map Level 4 Table Entry Controller
enum Pml4teController {
    Pml4e {
        pdpt: Box<Pdpt>,
        vaddr2pdpte_controller: BTreeMap<Vaddr, PdpteController>,
    },
    Pml4teNotPresent,
}

impl Pml4teController {
    fn copy(source: &Pml4te, destination: &mut Pml4te, vaddr: Vaddr) -> Self {
        match (source.pml4e(), source.pml4te_not_present()) {
            (Some(pml4e), None) => {
                let source: &Pdpt = pml4e.into();
                let mut pdpt: Box<Pdpt> = Box::default();
                destination.set_pml4e(*pml4e, pdpt.as_ref());
                let vaddr2pdpte_controller: BTreeMap<Vaddr, PdpteController> = source.pdpte
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
                        (vaddr, PdpteController::copy(source, destination, vaddr))
                    })
                    .collect();
                Self::Pml4e {
                    pdpt,
                    vaddr2pdpte_controller,
                }
            },
            (None, Some(pml4te_not_present)) => {
                destination.set_pml4te_not_present(*pml4te_not_present);
                Self::Pml4teNotPresent
            },
            _ => panic!("Can't get a page map level 4 table entry."),
        }
    }

    #[allow(dead_code)]
    fn debug(&self, vaddr: &Vaddr) {
        if let Self::Pml4e {
            pdpt,
            vaddr2pdpte_controller,
        } = self {
            let pdp_vaddr: Vaddr = vaddr
                .with_pdi(0)
                .with_pi(0)
                .with_offset(0);
            let pdpte: &Pdpte = pdpt
                .as_ref()
                .pdpte(&pdp_vaddr);
            com2_println!("pdpte = {:#x?}", pdpte);
            if let Some(pdpte_controller) = vaddr2pdpte_controller.get(&pdp_vaddr) {
                pdpte_controller.debug(vaddr);
            }
        }
    }

    fn set_page(&mut self, pml4te: &mut Pml4te, vaddr: &Vaddr, paddr: usize, present: bool, writable: bool, executable: bool) {
        if let Self::Pml4teNotPresent = self {
            let pdpt: Box<Pdpt> = Box::default();
            let vaddr2pdpte_controller: BTreeMap<Vaddr, PdpteController> = pdpt
                .as_ref()
                .pdpte
                .as_slice()
                .iter()
                .enumerate()
                .map(|(pdpi, _pdpte)| {
                    let vaddr: Vaddr = vaddr
                        .with_pdpi(pdpi as u16)
                        .with_pdi(0)
                        .with_pi(0)
                        .with_offset(0);
                    let pdpte_controller: PdpteController = PdpteController::default();
                    (vaddr, pdpte_controller)
                })
                .collect();
            let pml4e: Pml4e = Pml4e::default()
                .with_p(true)
                .with_rw(writable)
                .with_us(false)
                .with_pwt(false)
                .with_pcd(false)
                .with_a(false)
                .with_r(false)
                .with_xd(!executable);
            pml4te.set_pml4e(pml4e, pdpt.as_ref());
            *self = Self::Pml4e {
                pdpt,
                vaddr2pdpte_controller,
            };
        }
        if let Self::Pml4e {
            pdpt,
            vaddr2pdpte_controller,
        } = self {
            let old_pml4e: Pml4e = *pml4te
                .pml4e()
                .unwrap();
            let new_pml4e: Pml4e = old_pml4e
                .with_rw(old_pml4e.rw() || writable)
                .with_xd(old_pml4e.xd() && !executable);
            pml4te.set_pml4e(new_pml4e, pdpt.as_ref());
            let pdp_vaddr: Vaddr = vaddr
                .with_pdi(0)
                .with_pi(0)
                .with_offset(0);
            let pdpte: &mut Pdpte = pdpt
                .as_mut()
                .pdpte_mut(&pdp_vaddr);
            vaddr2pdpte_controller
                .get_mut(&pdp_vaddr)
                .unwrap()
                .set_page(pdpte, vaddr, paddr, present, writable, executable);
        } else {
            panic!("Can't set a page!");
        };
    }
}

impl fmt::Debug for Pml4teController {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pml4e {
                pdpt,
                vaddr2pdpte_controller,
            } => formatter
                .debug_map()
                .entries(vaddr2pdpte_controller
                    .iter()
                    .zip(pdpt
                        .as_ref()
                        .pdpte
                        .as_slice()
                        .iter())
                    .map(|((vaddr, pdpte_controller), pdpte)| (vaddr, (pdpte, pdpte_controller))))
                .finish(),
            Self::Pml4teNotPresent => formatter.write_str("Pml4teNotPresent"),
        }
    }
}

impl From<&Pml4te> for Pml4teController {
    fn from(pml4te: &Pml4te) -> Self {
        match (pml4te.pml4e(), pml4te.pml4te_not_present()) {
            (Some(pml4e), None) => {
                let pdpt: &Pdpt = pml4e.into();
                let pdpt = Box::<Pdpt>::new(pdpt.clone());
                let vaddr2pdpte_controller = BTreeMap::<Vaddr, PdpteController>::new();
                Self::Pml4e {
                    pdpt,
                    vaddr2pdpte_controller,
                }
            },
            (None, Some(pml4te_not_present)) => Self::Pml4teNotPresent,
            _ => panic!("Can't convert from &Pml4te to Pml4teController"),
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

impl Pml4te {
    fn pml4e(&self) -> Option<&Pml4e> {
        let pml4e: &Pml4e = unsafe {
            &self.pml4e
        };
        pml4e.p().then_some(pml4e)
    }

    fn pml4te_not_present(&self) -> Option<&Pml4teNotPresent> {
        let pml4te_not_present: &Pml4teNotPresent = unsafe {
            &self.pml4te_not_present
        };
        (!pml4te_not_present.p()).then_some(pml4te_not_present)
    }

    fn set_pml4e(&mut self, mut pml4e: Pml4e, pdpt: &Pdpt) {
        let pdpt: *const Pdpt = pdpt as *const Pdpt;
        let pdpt: u64 = pdpt as u64;
        let pdpt: u64 = pdpt >> Pml4e::ADDRESS_OF_PDPT_OFFSET;
        pml4e.set_p(true);
        pml4e.set_address_of_pdpt(pdpt);
        self.pml4e = pml4e;
        assert!(self.pml4e().is_some());
        assert!(self.pml4te_not_present().is_none());
    }

    fn set_pml4te_not_present(&mut self, pml4te_not_present: Pml4teNotPresent) {
        self.pml4te_not_present = pml4te_not_present;
        assert!(self.pml4e().is_none());
        assert!(self.pml4te_not_present().is_some());
    }
}

impl Default for Pml4te {
    fn default() -> Self {
        let pml4te_not_present = Pml4teNotPresent::default();
        Self {
            pml4te_not_present
        }
    }
}

impl fmt::Debug for Pml4te {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.pml4e(), self.pml4te_not_present()) {
            (Some(pml4e), None) => formatter
                .debug_struct("Pml4e")
                .field("p", &pml4e.p())
                .field("rw", &pml4e.rw())
                .field("us", &pml4e.us())
                .field("pwt", &pml4e.pwt())
                .field("pcd", &pml4e.pcd())
                .field("a", &pml4e.a())
                .field("r", &pml4e.r())
                .field("pdpt", &pml4e.pdpt())
                .field("xd", &pml4e.xd())
                .finish(),
            (None, Some(pml4te_not_present)) => formatter
                .debug_struct("Pml4teNotPresent")
                .field("p", &pml4te_not_present.p())
                .finish(),
            _ => panic!("Can't format a page map level 4 table entry."),
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

impl Pml4e {
    fn pdpt(&self) -> *const Pdpt {
        (self.address_of_pdpt() << Self::ADDRESS_OF_PDPT_OFFSET) as *const Pdpt
    }
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
#[derive(Clone)]
#[repr(align(4096))]
struct Pdpt {
    pdpte: [Pdpte; PDPT_LENGTH],
}

impl Pdpt {
    #[allow(dead_code)]
    fn pdpte(&self, vaddr: &Vaddr) -> &Pdpte {
        &self.pdpte[vaddr.pdpi() as usize]
    }

    fn pdpte_mut(&mut self, vaddr: &Vaddr) -> &mut Pdpte {
        &mut self.pdpte[vaddr.pdpi() as usize]
    }
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
        unsafe {
            &*pml4e.pdpt()
        }
    }
}

/// # Page Directory Pointer Table Entry Controller
enum PdpteController {
    Pe1Gib,
    Pdpe {
        pdt: Box<Pdt>,
        vaddr2pdte_controller: BTreeMap<Vaddr, PdteController>,
    },
    PdpteNotPresent,
}

impl PdpteController {
    fn copy(source: &Pdpte, destination: &mut Pdpte, vaddr: Vaddr) -> Self {
        match (source.pe1gib(), source.pdpe(), source.pdpte_not_present()) {
            (Some(pe1gib), None, None) => {
                destination.set_pe1gib(*pe1gib);
                Self::Pe1Gib
            },
            (None, Some(pdpe), None) => {
                let source: &Pdt = pdpe.into();
                let mut pdt: Box<Pdt> = Box::default();
                destination.set_pdpe(*pdpe, pdt.as_ref());
                let vaddr2pdte_controller: BTreeMap<Vaddr, PdteController> = source.pdte
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
                        (vaddr, PdteController::copy(source, destination, vaddr))
                    })
                    .collect();
                Self::Pdpe {
                    pdt,
                    vaddr2pdte_controller,
                }
            },
            (None, None, Some(pdpte_not_present)) => {
                destination.set_pdpte_not_present(*pdpte_not_present);
                Self::PdpteNotPresent
            },
            _ => panic!("Can't get a page directory pointer table entry."),
        }
    }

    #[allow(dead_code)]
    fn debug(&self, vaddr: &Vaddr) {
        if let Self::Pdpe {
            pdt,
            vaddr2pdte_controller,
        } = self {
            let pd_vaddr: Vaddr = vaddr
                .with_pi(0)
                .with_offset(0);
            let pdte: &Pdte = pdt
                .as_ref()
                .pdte(&pd_vaddr);
            com2_println!("pdte = {:#x?}", pdte);
            if let Some(pdte_controller) = vaddr2pdte_controller.get(&pd_vaddr) {
                pdte_controller.debug(vaddr);
            }
        }
    }

    fn set_page(&mut self, pdpte: &mut Pdpte, vaddr: &Vaddr, paddr: usize, present: bool, writable: bool, executable: bool) {
        match self {
            Self::Pe1Gib => {
                let pe1gib: Pe1Gib = *pdpte
                    .clone()
                    .pe1gib()
                    .unwrap();
                let page_1gib_paddr: usize = pe1gib.page_1gib() as usize;
                let mut pdt: Box<Pdt> = Box::default();
                let vaddr2pdte_controller: BTreeMap<Vaddr, PdteController> = pdt
                    .as_mut()
                    .pdte
                    .as_mut_slice()
                    .iter_mut()
                    .enumerate()
                    .map(|(pdi, pdte)| {
                        let vaddr: Vaddr = vaddr
                            .with_pdi(pdi as u16)
                            .with_pi(0)
                            .with_offset(0);
                        let pdte_controller: PdteController = PdteController::Pe2Mib;
                        let page_2mib_paddr: usize = page_1gib_paddr + (pdi << Pe2Mib::ADDRESS_OF_2MIB_PAGE_FRAME_OFFSET);
                        let pe2mib: Pe2Mib = Pe2Mib::default()
                            .with_p(true)
                            .with_rw(pe1gib.rw())
                            .with_us(pe1gib.us())
                            .with_pwt(pe1gib.pwt())
                            .with_pcd(pe1gib.pcd())
                            .with_a(false)
                            .with_d(pe1gib.d())
                            .with_is_page_2mib(true)
                            .with_g(pe1gib.g())
                            .with_r(pe1gib.r())
                            .with_pat(pe1gib.pat())
                            .with_address_of_2mib_page_frame((page_2mib_paddr >> Pe2Mib::ADDRESS_OF_2MIB_PAGE_FRAME_OFFSET) as u32)
                            .with_prot_key(pe1gib.prot_key())
                            .with_xd(pe1gib.xd());
                        pdte.set_pe2mib(pe2mib);
                        (vaddr, pdte_controller)
                    })
                    .collect();
                let pdpe: Pdpe = Pdpe::default()
                    .with_p(true)
                    .with_rw(pe1gib.rw() || writable)
                    .with_us(pe1gib.us())
                    .with_pwt(pe1gib.pwt())
                    .with_pcd(pe1gib.pcd())
                    .with_a(pe1gib.a())
                    .with_is_page_1gib(false)
                    .with_r(pe1gib.r())
                    .with_xd(pe1gib.xd() && !executable);
                pdpte.set_pdpe(pdpe, pdt.as_ref());
                *self = Self::Pdpe {
                    pdt,
                    vaddr2pdte_controller,
                };
            },
            Self::PdpteNotPresent => {
                let pdt: Box<Pdt> = Box::default();
                let vaddr2pdte_controller: BTreeMap<Vaddr, PdteController> = pdt
                    .as_ref()
                    .pdte
                    .as_slice()
                    .iter()
                    .enumerate()
                    .map(|(pdi, _pdte)| {
                        let vaddr: Vaddr = vaddr
                            .with_pdi(pdi as u16)
                            .with_pi(0)
                            .with_offset(0);
                        let pdte_controller: PdteController = PdteController::default();
                        (vaddr, pdte_controller)
                    })
                    .collect();
                let pdpe: Pdpe = Pdpe::default()
                    .with_p(true)
                    .with_rw(writable)
                    .with_us(false)
                    .with_pwt(false)
                    .with_pcd(false)
                    .with_a(false)
                    .with_is_page_1gib(false)
                    .with_r(false)
                    .with_xd(!executable);
                pdpte.set_pdpe(pdpe, pdt.as_ref());
                *self = Self::Pdpe {
                    pdt,
                    vaddr2pdte_controller,
                };
            },
            _ => {},
        }
        if let Self::Pdpe {
            pdt,
            vaddr2pdte_controller,
        } = self {
            let old_pdpe: Pdpe = *pdpte
                .pdpe()
                .unwrap();
            let new_pdpe: Pdpe = old_pdpe
                .with_rw(old_pdpe.rw() || writable)
                .with_xd(old_pdpe.xd() && !executable);
            pdpte.set_pdpe(new_pdpe, pdt.as_ref());
            let pd_vaddr: Vaddr = vaddr
                .with_pi(0)
                .with_offset(0);
            let pdte: &mut Pdte = pdt
                .as_mut()
                .pdte_mut(&pd_vaddr);
            vaddr2pdte_controller
                .get_mut(&pd_vaddr)
                .unwrap()
                .set_page(pdte, vaddr, paddr, present, writable, executable);
        } else {
            panic!("Can't set a page!");
        }
    }
}

impl Default for PdpteController {
    fn default() -> Self {
        Self::PdpteNotPresent
    }
}

impl fmt::Debug for PdpteController {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pe1Gib => formatter.write_str("Pe1Gib"),
            Self::Pdpe {
                pdt,
                vaddr2pdte_controller,
            } => formatter
                .debug_map()
                .entries(vaddr2pdte_controller
                    .iter()
                    .zip(pdt
                        .as_ref()
                        .pdte
                        .as_slice()
                        .iter())
                    .map(|((vaddr, pdte_controller), pdte)| (vaddr, (pdte, pdte_controller))))
                .finish(),
            Self::PdpteNotPresent => formatter.write_str("PdpteNotPresent"),
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
        (pe1gib.p() && pe1gib.is_page_1gib()).then_some(pe1gib)
    }

    fn pdpe(&self) -> Option<&Pdpe> {
        let pdpe: &Pdpe = unsafe {
            &self.pdpe
        };
        (pdpe.p() && !pdpe.is_page_1gib()).then_some(pdpe)
    }

    fn pdpte_not_present(&self) -> Option<&PdpteNotPresent> {
        let pdpte_not_present: &PdpteNotPresent = unsafe {
            &self.pdpte_not_present
        };
        (!pdpte_not_present.p()).then_some(pdpte_not_present)
    }

    fn set_pe1gib(&mut self, pe1gib: Pe1Gib) {
        self.pe1gib = pe1gib;
        assert!(self.pe1gib().is_some());
        assert!(self.pdpe().is_none());
        assert!(self.pdpte_not_present().is_none());
    }

    fn set_pdpe(&mut self, mut pdpe: Pdpe, pdt: &Pdt) {
        let pdt: *const Pdt = pdt as *const Pdt;
        let pdt: u64 = pdt as u64;
        let pdt: u64 = pdt >> Pdpe::ADDRESS_OF_PDT_OFFSET;
        pdpe.set_address_of_pdt(pdt);
        self.pdpe = pdpe;
        assert!(self.pe1gib().is_none());
        assert!(self.pdpe().is_some());
        assert!(self.pdpte_not_present().is_none());
    }

    fn set_pdpte_not_present(&mut self, pdpte_not_present: PdpteNotPresent) {
        self.pdpte_not_present = pdpte_not_present;
        assert!(self.pe1gib().is_none());
        assert!(self.pdpe().is_none());
        assert!(self.pdpte_not_present().is_some());
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

impl fmt::Debug for Pdpte {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.pe1gib(), self.pdpe(), self.pdpte_not_present()) {
            (Some(pe1gib), None, None) => formatter
                .debug_struct("Pe1Gib")
                .field("p", &pe1gib.p())
                .field("rw", &pe1gib.rw())
                .field("us", &pe1gib.us())
                .field("pwt", &pe1gib.pwt())
                .field("pcd", &pe1gib.pcd())
                .field("a", &pe1gib.a())
                .field("d", &pe1gib.d())
                .field("is_page_1gib", &pe1gib.is_page_1gib())
                .field("g", &pe1gib.g())
                .field("r", &pe1gib.r())
                .field("pat", &pe1gib.pat())
                .field("page_1gib", &pe1gib.page_1gib())
                .field("prot_key", &pe1gib.prot_key())
                .field("xd", &pe1gib.xd())
                .finish(),
            (None, Some(pdpe), None) => formatter
                .debug_struct("Pdpe")
                .field("p", &pdpe.p())
                .field("rw", &pdpe.rw())
                .field("us", &pdpe.us())
                .field("pwt", &pdpe.pwt())
                .field("pcd", &pdpe.pcd())
                .field("a", &pdpe.a())
                .field("is_page_1gib", &pdpe.is_page_1gib())
                .field("r", &pdpe.r())
                .field("pdt", &pdpe.pdt())
                .field("xd", &pdpe.xd())
                .finish(),
            (None, None, Some(pdpte_not_present)) => formatter
                .debug_struct("PdpteNotPresent")
                .field("p", &pdpte_not_present.p())
                .finish(),
            _ => panic!("Can't format a page directory pointer table entry."),
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
    is_page_1gib: bool,
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

impl Pe1Gib {
    fn page_1gib(&self) -> *const Page1Gib {
        (self.address_of_1gib_page_frame() << Self::ADDRESS_OF_1GIB_PAGE_FRAME_OFFSET) as *const Page1Gib
    }
}

type Page1Gib = [u8; 1 << Pe1Gib::ADDRESS_OF_1GIB_PAGE_FRAME_OFFSET];

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
    is_page_1gib: bool,
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
    fn pdt(&self) -> *const Pdt {
        (self.address_of_pdt() << Self::ADDRESS_OF_PDT_OFFSET) as *const Pdt
    }
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

impl Pdt {
    #[allow(dead_code)]
    fn pdte(&self, vaddr: &Vaddr) -> &Pdte {
        &self.pdte[vaddr.pdi() as usize]
    }

    fn pdte_mut(&mut self, vaddr: &Vaddr) -> &mut Pdte {
        &mut self.pdte[vaddr.pdi() as usize]
    }
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
        unsafe {
            &*pdpe.pdt()
        }
    }
}

/// # Page Directory Table Entry Controller
enum PdteController {
    Pe2Mib,
    Pde {
        pt: Box<Pt>,
        vaddr2pte_controller: BTreeMap<Vaddr, PteController>,
    },
    PdteNotPresent,
}

impl PdteController {
    fn copy(source: &Pdte, destination: &mut Pdte, vaddr: Vaddr) -> Self {
        match (source.pe2mib(), source.pde(), source.pdte_not_present()) {
            (Some(pe2mib), None, None) => {
                destination.set_pe2mib(*pe2mib);
                Self::Pe2Mib
            },
            (None, Some(pde), None) => {
                let source: &Pt = pde.into();
                let mut pt: Box<Pt> = Box::default();
                destination.set_pde(*pde, pt.as_ref());
                let vaddr2pte_controller: BTreeMap<Vaddr, PteController> = source.pte
                    .as_slice()
                    .iter()
                    .zip(pt
                        .as_mut()
                        .pte
                        .as_mut_slice()
                        .iter_mut())
                    .enumerate()
                    .map(|(pi, (source, destination))| (vaddr.with_pi(pi as u16), PteController::copy(source, destination)))
                    .collect();
                Self::Pde {
                    pt,
                    vaddr2pte_controller,
                }
            },
            (None, None, Some(pdte_not_present)) => {
                destination.set_pdte_not_present(*pdte_not_present);
                Self::PdteNotPresent
            },
            _ => panic!("Can't get a page directory table entry."),
        }
    }

    #[allow(dead_code)]
    fn debug(&self, vaddr: &Vaddr) {
        if let Self::Pde {
            pt,
            vaddr2pte_controller: _,
        } = self {
            let p_vaddr: Vaddr = vaddr
                .with_offset(0);
            let pte: &Pte = pt
                .as_ref()
                .pte(&p_vaddr);
            com2_println!("pte = {:#x?}", pte);
        }
    }

    fn set_page(&mut self, pdte: &mut Pdte, vaddr: &Vaddr, paddr: usize, present: bool, writable: bool, executable: bool) {
        match self {
            Self::Pe2Mib => {
                let pe2mib: Pe2Mib = *pdte
                    .clone()
                    .pe2mib()
                    .unwrap();
                let page_2mib_paddr: usize = pe2mib.page_2mib() as usize;
                let mut pt: Box<Pt> = Box::default();
                let vaddr2pte_controller: BTreeMap<Vaddr, PteController> = pt
                    .as_mut()
                    .pte
                    .as_mut_slice()
                    .iter_mut()
                    .enumerate()
                    .map(|(pi, pte)| {
                        let vaddr: Vaddr = vaddr
                            .with_pi(pi as u16)
                            .with_offset(0);
                        let pte_controller: PteController = PteController::Pe4Kib;
                        let page_4kib_paddr: usize = page_2mib_paddr + (pi << Pe4Kib::ADDRESS_OF_4KIB_PAGE_FRAME_OFFSET);
                        let pe4kib: Pe4Kib = Pe4Kib::default()
                            .with_p(true)
                            .with_rw(pe2mib.rw())
                            .with_us(pe2mib.us())
                            .with_pwt(pe2mib.pwt())
                            .with_pcd(pe2mib.pcd())
                            .with_a(false)
                            .with_d(pe2mib.d())
                            .with_pat(pe2mib.pat())
                            .with_g(pe2mib.g())
                            .with_r(pe2mib.r())
                            .with_address_of_4kib_page_frame((page_4kib_paddr >> Pe4Kib::ADDRESS_OF_4KIB_PAGE_FRAME_OFFSET) as u64)
                            .with_prot_key(pe2mib.prot_key())
                            .with_xd(pe2mib.xd());
                        pte.set_pe4kib(pe4kib);
                        (vaddr, pte_controller)
                    })
                    .collect();
                let pde: Pde = Pde::default()
                    .with_p(true)
                    .with_rw(writable)
                    .with_us(false)
                    .with_pwt(false)
                    .with_pcd(false)
                    .with_a(false)
                    .with_is_page_2mib(false)
                    .with_r(false)
                    .with_xd(pe2mib.xd() && !executable);
                pdte.set_pde(pde, pt.as_ref());
                *self = Self::Pde {
                    pt,
                    vaddr2pte_controller,
                }
            },
            Self::PdteNotPresent => {
                let pt: Box<Pt> = Box::default();
                let vaddr2pte_controller: BTreeMap<Vaddr, PteController> = pt
                    .as_ref()
                    .pte
                    .as_slice()
                    .iter()
                    .enumerate()
                    .map(|(pi, _pte)| {
                        let vaddr: Vaddr = vaddr
                            .with_pi(pi as u16)
                            .with_offset(0);
                        let pte_controller: PteController = PteController::default();
                        (vaddr, pte_controller)
                    })
                    .collect();
                let pde: Pde = Pde::default()
                    .with_p(true)
                    .with_rw(writable)
                    .with_us(false)
                    .with_pwt(false)
                    .with_pcd(false)
                    .with_a(false)
                    .with_is_page_2mib(false)
                    .with_r(false)
                    .with_xd(!executable);
                pdte.set_pde(pde, pt.as_ref());
                *self = Self::Pde {
                    pt,
                    vaddr2pte_controller,
                };
            },
            _ => {},
        }
        if let Self::Pde {
            pt,
            vaddr2pte_controller,
        } = self {
            let old_pde: Pde = *pdte
                .pde()
                .unwrap();
            let new_pde: Pde = old_pde
                .with_rw(old_pde.rw() || writable)
                .with_xd(old_pde.xd() && !executable);
            pdte.set_pde(new_pde, pt.as_ref());
            let p_vaddr: Vaddr = vaddr
                .with_offset(0);
            let pte: &mut Pte = pt
                .as_mut()
                .pte_mut(&p_vaddr);
            vaddr2pte_controller
                .get_mut(&p_vaddr)
                .unwrap()
                .set_page(pte, paddr, present, writable, executable);
        } else {
            panic!("Can't set a page!");
        }
    }
}

impl Default for PdteController {
    fn default() -> Self {
        Self::PdteNotPresent
    }
}

impl fmt::Debug for PdteController {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pe2Mib => formatter.write_str("Pe2Mib"),
            Self::Pde {
                pt,
                vaddr2pte_controller,
            } => formatter
                .debug_map()
                .entries(vaddr2pte_controller
                    .iter()
                    .zip(pt
                        .as_ref()
                        .pte
                        .as_slice()
                        .iter())
                    .map(|((vaddr, pte_controller), pte)| (vaddr, (pte, pte_controller))))
                .finish(),
            Self::PdteNotPresent => formatter.write_str("PdteNotPresent"),
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
        (pe2mib.p() && pe2mib.is_page_2mib()).then_some(pe2mib)
    }

    fn pde(&self) -> Option<&Pde> {
        let pde: &Pde = unsafe {
            &self.pde
        };
        (pde.p() && !pde.is_page_2mib()).then_some(pde)
    }

    fn pdte_not_present(&self) -> Option<&PdteNotPresent> {
        let pdte_not_present: &PdteNotPresent = unsafe {
            &self.pdte_not_present
        };
        (!pdte_not_present.p()).then_some(pdte_not_present)
    }

    fn set_pe2mib(&mut self, pe2mib: Pe2Mib) {
        self.pe2mib = pe2mib;
        assert!(self.pe2mib().is_some());
        assert!(self.pde().is_none());
        assert!(self.pdte_not_present().is_none());
    }

    fn set_pde(&mut self, mut pde: Pde, pt: &Pt) {
        let pt: *const Pt = pt as *const Pt;
        let pt: u64 = pt as u64;
        let pt: u64 = pt >> Pde::ADDRESS_OF_PT_OFFSET;
        pde.set_address_of_pt(pt);
        self.pde = pde;
        assert!(self.pe2mib().is_none());
        assert!(self.pde().is_some());
        assert!(self.pdte_not_present().is_none());
    }

    fn set_pdte_not_present(&mut self, pdte_not_present: PdteNotPresent) {
        self.pdte_not_present = pdte_not_present;
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

impl fmt::Debug for Pdte {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.pe2mib(), self.pde(), self.pdte_not_present()) {
            (Some(pe2mib), None, None) => formatter
                .debug_struct("Pe2Mib")
                .field("p", &pe2mib.p())
                .field("rw", &pe2mib.rw())
                .field("us", &pe2mib.us())
                .field("pwt", &pe2mib.pwt())
                .field("pcd", &pe2mib.pcd())
                .field("a", &pe2mib.a())
                .field("d", &pe2mib.d())
                .field("is_page_2mib", &pe2mib.is_page_2mib())
                .field("g", &pe2mib.g())
                .field("r", &pe2mib.r())
                .field("pat", &pe2mib.pat())
                .field("page_2mib", &pe2mib.page_2mib())
                .field("prot_key", &pe2mib.prot_key())
                .field("xd", &pe2mib.xd())
                .finish(),
            (None, Some(pde), None) => formatter
                .debug_struct("Pde")
                .field("p", &pde.p())
                .field("rw", &pde.rw())
                .field("us", &pde.us())
                .field("pwt", &pde.pwt())
                .field("pcd", &pde.pcd())
                .field("a", &pde.a())
                .field("is_page_2mib", &pde.is_page_2mib())
                .field("r", &pde.r())
                .field("pt", &pde.pt())
                .field("xd", &pde.xd())
                .finish(),
            (None, None, Some(pdte_not_present)) => formatter
                .debug_struct("PdteNotPresent")
                .field("p", &pdte_not_present.p())
                .finish(),
            _ => panic!("Can't format a page directory table entry."),
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
    is_page_2mib: bool,
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

impl Pe2Mib {
    fn page_2mib(&self) -> *const Page2Mib {
        (self.address_of_2mib_page_frame() << Self::ADDRESS_OF_2MIB_PAGE_FRAME_OFFSET) as *const Page2Mib
    }
}

type Page2Mib = [u8; 1 << Pe2Mib::ADDRESS_OF_2MIB_PAGE_FRAME_OFFSET];

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
    is_page_2mib: bool,
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
    fn pt(&self) -> *const Pt {
        (self.address_of_pt() << Self::ADDRESS_OF_PT_OFFSET) as *const Pt
    }
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
#[derive(Debug)]
#[repr(align(4096))]
struct Pt {
    pte: [Pte; PT_LENGTH]
}

impl Pt {
    #[allow(dead_code)]
    fn pte(&self, vaddr: &Vaddr) -> &Pte {
        &self.pte[vaddr.pi() as usize]
    }

    fn pte_mut(&mut self, vaddr: &Vaddr) -> &mut Pte {
        &mut self.pte[vaddr.pi() as usize]
    }
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
        unsafe {
            &*pde.pt()
        }
    }
}

/// # Page Table Entry Controller
#[derive(Debug)]
enum PteController {
    Pe4Kib,
    PteNotPresent,
}

impl PteController {
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

    fn set_page(&mut self, pte: &mut Pte, paddr: usize, present: bool, writable: bool, executable: bool) {
        if present {
            let pe4kib: Pe4Kib = Pe4Kib::default()
                .with_p(true)
                .with_rw(writable)
                .with_us(false)
                .with_pwt(false)
                .with_pcd(false)
                .with_a(false)
                .with_d(false)
                .with_pat(false)
                .with_g(false)
                .with_r(false)
                .with_address_of_4kib_page_frame((paddr >> Pe4Kib::ADDRESS_OF_4KIB_PAGE_FRAME_OFFSET) as u64)
                .with_prot_key(0)
                .with_xd(!executable);
            pte.set_pe4kib(pe4kib);
            *self = Self::Pe4Kib;
        } else {
            let pte_not_present = PteNotPresent::default();
            pte.set_pte_not_present(pte_not_present);
            *self = Self::PteNotPresent;
        }
    }
}

impl Default for PteController {
    fn default() -> Self {
        Self::PteNotPresent
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
        pe4kib.p().then_some(pe4kib)
    }

    fn pte_not_present(&self) -> Option<&PteNotPresent> {
        let pte_not_present: &PteNotPresent = unsafe {
            &self.pte_not_present
        };
        (!pte_not_present.p()).then_some(pte_not_present)
    }

    fn set_pe4kib(&mut self, pe4kib: Pe4Kib) {
        self.pe4kib = pe4kib;
        assert!(self.pe4kib().is_some());
        assert!(self.pte_not_present().is_none());
    }

    fn set_pte_not_present(&mut self, pte_not_present: PteNotPresent) {
        self.pte_not_present = pte_not_present;
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

impl fmt::Debug for Pte {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.pe4kib(), self.pte_not_present()) {
            (Some(pe4kib), None) => formatter
                .debug_struct("Pe4Kib")
                .field("p", &pe4kib.p())
                .field("rw", &pe4kib.rw())
                .field("us", &pe4kib.us())
                .field("pwt", &pe4kib.pwt())
                .field("pcd", &pe4kib.pcd())
                .field("a", &pe4kib.a())
                .field("d", &pe4kib.d())
                .field("pat", &pe4kib.pat())
                .field("g", &pe4kib.g())
                .field("r", &pe4kib.r())
                .field("page_4kib", &pe4kib.page_4kib())
                .field("prot_key", &pe4kib.prot_key())
                .field("xd", &pe4kib.xd())
                .finish(),
            (None, Some(pte_not_present)) => formatter
                .debug_struct("PteNotPresent")
                .field("p", &pte_not_present.p())
                .finish(),
            _ => panic!("Can't format a page table entry."),
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
    address_of_4kib_page_frame: u64,
    #[bits(11, access = RO)]
    reserved1: u16,
    #[bits(4)]
    prot_key: u8,
    xd: bool,
}

impl Pe4Kib {
    fn page_4kib(&self) -> *const Page4Mib {
        (self.address_of_4kib_page_frame() << Self::ADDRESS_OF_4KIB_PAGE_FRAME_OFFSET) as *const Page4Mib
    }
}

type Page4Mib = [u8; 1 << Pe4Kib::ADDRESS_OF_4KIB_PAGE_FRAME_OFFSET];

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

impl From<usize> for Vaddr {
    fn from(vaddr: usize) -> Self {
        let vaddr: u64 = vaddr as u64;
        vaddr.into()
    }
}

impl From<Vaddr> for usize {
    fn from(vaddr: Vaddr) -> Self {
        let vaddr: u64 = vaddr.into();
        vaddr as Self
    }
}

