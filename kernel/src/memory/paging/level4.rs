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

pub struct Interface {
    cr3: x64::control::Register3,
    pml4t: Box<Pml4t>,
    vaddr2pml4te_interface: BTreeMap<Vaddr, Pml4teInterface>,
}

impl Interface {
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
        if let Some(pml4te_interface) = self.vaddr2pml4te_interface.get(&pml4vaddr) {
            pml4te_interface.debug(&vaddr);
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

    #[allow(dead_code)]
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
        let pml4te: &mut Pml4te = self.pml4t
            .as_mut()
            .pml4te_mut(&pml4vaddr);
        self.vaddr2pml4te_interface
            .get_mut(&pml4vaddr)
            .unwrap()
            .set_page(pml4te, &vaddr, paddr, present, writable, executable);
    }

    pub fn vaddr2paddr(&self, vaddr: usize) -> Option<usize> {
        let vaddr: Vaddr = vaddr.into();
        let pml4vaddr: Vaddr = vaddr
                .with_pdpi(0)
                .with_pdi(0)
                .with_pi(0)
                .with_offset(0);
        self.vaddr2pml4te_interface
            .get(&pml4vaddr)
            .and_then(|pml4te_interface| pml4te_interface.vaddr2paddr(&vaddr))
    }
}

impl fmt::Debug for Interface {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_map()
            .entries(self.vaddr2pml4te_interface
                .iter()
                .zip(self.pml4t
                    .as_ref()
                    .pml4te
                    .as_slice()
                    .iter())
                .map(|((vaddr, pml4te_interface), pml4te)| (vaddr, (pml4te, pml4te_interface))))
            .finish()
    }
}

/// # Page Map Level 4 Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(align(4096))]
struct Pml4t {
    pml4te: [Pml4te; PML4T_LENGTH],
}

impl Pml4t {
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

/// # Page Map Level 4 Table Entry Interface
enum Pml4teInterface {
    Pml4e {
        pdpt: Box<Pdpt>,
        vaddr2pdpte_interface: BTreeMap<Vaddr, PdpteInterface>,
    },
    #[allow(dead_code)]
    Pml4teNotPresent,
}

impl Pml4teInterface {
    #[allow(dead_code)]
    fn debug(&self, vaddr: &Vaddr) {
        if let Self::Pml4e {
            pdpt,
            vaddr2pdpte_interface,
        } = self {
            let pdp_vaddr: Vaddr = vaddr
                .with_pdi(0)
                .with_pi(0)
                .with_offset(0);
            let pdpte: &Pdpte = pdpt
                .as_ref()
                .pdpte(&pdp_vaddr);
            com2_println!("pdpte = {:#x?}", pdpte);
            if let Some(pdpte_interface) = vaddr2pdpte_interface.get(&pdp_vaddr) {
                pdpte_interface.debug(vaddr);
            }
        }
    }

    fn set_page(&mut self, pml4te: &mut Pml4te, vaddr: &Vaddr, paddr: usize, present: bool, writable: bool, executable: bool) {
        if let Self::Pml4teNotPresent = self {
            let pdpt: Box<Pdpt> = Box::default();
            let vaddr2pdpte_interface: BTreeMap<Vaddr, PdpteInterface> = pdpt
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
                    let pdpte_interface: PdpteInterface = PdpteInterface::default();
                    (vaddr, pdpte_interface)
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
                vaddr2pdpte_interface,
            };
        }
        if let Self::Pml4e {
            pdpt,
            vaddr2pdpte_interface,
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
            vaddr2pdpte_interface
                .get_mut(&pdp_vaddr)
                .unwrap()
                .set_page(pdpte, vaddr, paddr, present, writable, executable);
        } else {
            panic!("Can't set a page!");
        };
    }

    fn vaddr2paddr(&self, vaddr: &Vaddr) -> Option<usize> {
        match self {
            Self::Pml4e {
                pdpt,
                vaddr2pdpte_interface,
            } => {
                let pdp_vaddr: Vaddr = vaddr
                    .with_pdi(0)
                    .with_pi(0)
                    .with_offset(0);
                let pdpte: &Pdpte = pdpt
                    .as_ref()
                    .pdpte(&pdp_vaddr);
                vaddr2pdpte_interface
                    .get(&pdp_vaddr)
                    .and_then(|pdpte_interface| pdpte_interface.vaddr2paddr(pdpte, vaddr))
            },
            Self::Pml4teNotPresent => None,
        }
    }
}

impl fmt::Debug for Pml4teInterface {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pml4e {
                pdpt,
                vaddr2pdpte_interface,
            } => formatter
                .debug_map()
                .entries(vaddr2pdpte_interface
                    .iter()
                    .zip(pdpt
                        .as_ref()
                        .pdpte
                        .as_slice()
                        .iter())
                    .map(|((vaddr, pdpte_interface), pdpte)| (vaddr, (pdpte, pdpte_interface))))
                .finish(),
            Self::Pml4teNotPresent => formatter.write_str("Pml4teNotPresent"),
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

    #[allow(dead_code)]
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
#[repr(align(4096))]
struct Pdpt {
    pdpte: [Pdpte; PDPT_LENGTH],
}

impl Pdpt {
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

/// # Page Directory Pointer Table Entry Interface
enum PdpteInterface {
    #[allow(dead_code)]
    Pe1Gib,
    Pdpe {
        pdt: Box<Pdt>,
        vaddr2pdte_interface: BTreeMap<Vaddr, PdteInterface>,
    },
    PdpteNotPresent,
}

impl PdpteInterface {
    #[allow(dead_code)]
    fn debug(&self, vaddr: &Vaddr) {
        if let Self::Pdpe {
            pdt,
            vaddr2pdte_interface,
        } = self {
            let pd_vaddr: Vaddr = vaddr
                .with_pi(0)
                .with_offset(0);
            let pdte: &Pdte = pdt
                .as_ref()
                .pdte(&pd_vaddr);
            com2_println!("pdte = {:#x?}", pdte);
            if let Some(pdte_interface) = vaddr2pdte_interface.get(&pd_vaddr) {
                pdte_interface.debug(vaddr);
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
                let vaddr2pdte_interface: BTreeMap<Vaddr, PdteInterface> = pdt
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
                        let pdte_interface: PdteInterface = PdteInterface::Pe2Mib;
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
                        (vaddr, pdte_interface)
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
                    vaddr2pdte_interface,
                };
            },
            Self::PdpteNotPresent => {
                let pdt: Box<Pdt> = Box::default();
                let vaddr2pdte_interface: BTreeMap<Vaddr, PdteInterface> = pdt
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
                        let pdte_interface: PdteInterface = PdteInterface::default();
                        (vaddr, pdte_interface)
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
                    vaddr2pdte_interface,
                };
            },
            _ => {},
        }
        if let Self::Pdpe {
            pdt,
            vaddr2pdte_interface,
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
            vaddr2pdte_interface
                .get_mut(&pd_vaddr)
                .unwrap()
                .set_page(pdte, vaddr, paddr, present, writable, executable);
        } else {
            panic!("Can't set a page!");
        }
    }

    fn vaddr2paddr(&self, pdpte: &Pdpte, vaddr: &Vaddr) -> Option<usize> {
        match self {
            Self::Pe1Gib => {
                let frame_base: usize = pdpte
                    .pe1gib()
                    .unwrap()
                    .page_1gib() as usize;
                let paddr: usize = frame_base
                    + ((vaddr.pdi() as usize) << Vaddr::PDI_OFFSET)
                    + ((vaddr.pi() as usize) << Vaddr::PI_OFFSET)
                    + (vaddr.offset() as usize);
                Some(paddr)
            },
            Self::Pdpe {
                pdt,
                vaddr2pdte_interface,
            } => {
                let pd_vaddr: Vaddr = vaddr
                    .with_pi(0)
                    .with_offset(0);
                let pdte: &Pdte = pdt
                    .as_ref()
                    .pdte(&pd_vaddr);
                vaddr2pdte_interface
                    .get(&pd_vaddr)
                    .and_then(|pdte_interface| pdte_interface.vaddr2paddr(pdte, vaddr))
            },
            Self::PdpteNotPresent => None,
        }
    }
}

impl Default for PdpteInterface {
    fn default() -> Self {
        Self::PdpteNotPresent
    }
}

impl fmt::Debug for PdpteInterface {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pe1Gib => formatter.write_str("Pe1Gib"),
            Self::Pdpe {
                pdt,
                vaddr2pdte_interface,
            } => formatter
                .debug_map()
                .entries(vaddr2pdte_interface
                    .iter()
                    .zip(pdt
                        .as_ref()
                        .pdte
                        .as_slice()
                        .iter())
                    .map(|((vaddr, pdte_interface), pdte)| (vaddr, (pdte, pdte_interface))))
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
    #[allow(dead_code)]
    fn debug(&self, vaddr: &Vaddr) {
        if let Self::Pde {
            pt,
            vaddr2pte_interface: _,
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
                let vaddr2pte_interface: BTreeMap<Vaddr, PteInterface> = pt
                    .as_mut()
                    .pte
                    .as_mut_slice()
                    .iter_mut()
                    .enumerate()
                    .map(|(pi, pte)| {
                        let vaddr: Vaddr = vaddr
                            .with_pi(pi as u16)
                            .with_offset(0);
                        let pte_interface: PteInterface = PteInterface::Pe4Kib;
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
                        (vaddr, pte_interface)
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
                    vaddr2pte_interface,
                }
            },
            Self::PdteNotPresent => {
                let pt: Box<Pt> = Box::default();
                let vaddr2pte_interface: BTreeMap<Vaddr, PteInterface> = pt
                    .as_ref()
                    .pte
                    .as_slice()
                    .iter()
                    .enumerate()
                    .map(|(pi, _pte)| {
                        let vaddr: Vaddr = vaddr
                            .with_pi(pi as u16)
                            .with_offset(0);
                        let pte_interface: PteInterface = PteInterface::default();
                        (vaddr, pte_interface)
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
                    vaddr2pte_interface,
                };
            },
            _ => {},
        }
        if let Self::Pde {
            pt,
            vaddr2pte_interface,
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
            vaddr2pte_interface
                .get_mut(&p_vaddr)
                .unwrap()
                .set_page(pte, paddr, present, writable, executable);
        } else {
            panic!("Can't set a page!");
        }
    }

    fn vaddr2paddr(&self, pdte: &Pdte, vaddr: &Vaddr) -> Option<usize> {
        match self {
            Self::Pe2Mib => {
                let frame_base: usize = pdte
                    .pe2mib()
                    .unwrap()
                    .page_2mib() as usize;
                let paddr: usize = frame_base
                    + ((vaddr.pi() as usize) << Vaddr::PI_OFFSET)
                    + (vaddr.offset() as usize);
                Some(paddr)
            },
            Self::Pde {
                pt,
                vaddr2pte_interface,
            } => {
                let p_vaddr: Vaddr = vaddr
                    .with_offset(0);
                let pte: &Pte = pt
                    .as_ref()
                    .pte(&p_vaddr);
                vaddr2pte_interface
                    .get(&p_vaddr)
                    .and_then(|pte_interface| pte_interface.vaddr2paddr(pte, vaddr))
            },
            Self::PdteNotPresent => None,
        }
    }
}

impl Default for PdteInterface {
    fn default() -> Self {
        Self::PdteNotPresent
    }
}

impl fmt::Debug for PdteInterface {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pe2Mib => formatter.write_str("Pe2Mib"),
            Self::Pde {
                pt,
                vaddr2pte_interface,
            } => formatter
                .debug_map()
                .entries(vaddr2pte_interface
                    .iter()
                    .zip(pt
                        .as_ref()
                        .pte
                        .as_slice()
                        .iter())
                    .map(|((vaddr, pte_interface), pte)| (vaddr, (pte, pte_interface))))
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

    #[allow(dead_code)]
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

/// # Page Table Entry Interface
#[derive(Debug)]
enum PteInterface {
    Pe4Kib,
    PteNotPresent,
}

impl PteInterface {
    fn set_page(&mut self, pte: &mut Pte, paddr: usize, present: bool, writable: bool, executable: bool) {
        if present {
            let pe4kib: Pe4Kib = Pe4Kib::default()
                .with_p(present)
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
            *self = Self::PteNotPresent;
        }
    }

    fn vaddr2paddr(&self, pte: &Pte, vaddr: &Vaddr) -> Option<usize> {
        match self {
            Self::Pe4Kib => {
                let frame_base: usize = pte
                    .pe4kib()
                    .unwrap()
                    .page_4kib() as usize;
                let paddr: usize = frame_base
                    + (vaddr.offset() as usize);
                Some(paddr)
            },
            Self::PteNotPresent => None,
        }
    }
}

impl Default for PteInterface {
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn paddr(&self) -> Option<usize> {
        let cr3 = x64::control::Register3::get();
        let pml4t: &Pml4t = (&cr3).into();
        let pml4te: &Pml4te = pml4t.pml4te(self);
        pml4te
            .pml4e()
            .and_then(|pml4e| {
                let pdpt: *const Pdpt = pml4e.pdpt();
                let pdpt: &Pdpt = unsafe {
                    &*pdpt
                };
                let pdpte: &Pdpte = pdpt.pdpte(self);
                pdpte
                    .pe1gib()
                    .map(|pe1gib| {
                        let page_1gib: usize = pe1gib.page_1gib() as usize;
                        let pdi: usize = (self.pdi() << Self::PDI_OFFSET) as usize;
                        let pi: usize = (self.pi() << Self::PI_OFFSET) as usize;
                        let offset: usize = self.offset() as usize;
                        page_1gib + pdi + pi + offset
                    })
                    .or(pdpte
                        .pdpe()
                        .and_then(|pdpe| {
                            let pdt: *const Pdt = pdpe.pdt();
                            let pdt: &Pdt = unsafe {
                                &*pdt
                            };
                            let pdte: &Pdte = pdt.pdte(self);
                            pdte
                                .pe2mib()
                                .map(|pe2mib| {
                                    let page_2mib: usize = pe2mib.page_2mib() as usize;
                                    let pi: usize = (self.pi() << Self::PI_OFFSET) as usize;
                                    let offset: usize = self.offset() as usize;
                                    page_2mib + pi + offset
                                })
                                .or(pdte
                                    .pde()
                                    .and_then(|pde| {
                                        let pt: *const Pt = pde.pt();
                                        let pt: &Pt = unsafe {
                                            &*pt
                                        };
                                        let pte: &Pte = pt.pte(self);
                                        pte
                                            .pe4kib()
                                            .map(|pe4kib| {
                                                let page_4kib: usize = pe4kib.page_4kib() as usize;
                                                let offset: usize = self.offset() as usize;
                                                page_4kib + offset
                                            })
                                    }))
                        }))
            })
    }
}

impl<T> From<&T> for Vaddr {
    fn from(object: &T) -> Self {
        let object: *const T = object as *const T;
        let object: usize = object as usize;
        object.into()
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

