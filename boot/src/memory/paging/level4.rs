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
    crate::{
        memory,
        x64,
    },
};

const PML4T_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Pml4e>();
const PDPT_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Pdpe>();
const PDT_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Pde>();
const PT_LENGTH: usize = memory::PAGE_SIZE / mem::size_of::<Pe4Kib>();

/// # Page Map Level 4 Table
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(align(4096))]
struct Pml4t {
    pml4te: [Pml4te; PML4T_LENGTH],
}

/// # Page Map Level 4 Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(C)]
union Pml4te {
    pml4e: Pml4e,
    pml4te_not_present: Pml4teNotPresent,
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

/// # Page Directory Pointer Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(C)]
union Pdpte {
    pe1gib: Pe1Gib,
    pdpe: Pdpe,
    pdpte_not_present: PdpteNotPresent,
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

/// # Page Directory Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(C)]
union Pdte {
    pe2mib: Pe2Mib,
    pde: Pde,
    pdte_not_present: PdteNotPresent,
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

/// # Page Table Entry
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 4-32 Figure 4-11. Formats of CR3 and Paging-Structure Entries with 4-Level Paging and 5-Level Paging
#[repr(C)]
union Pte {
    pe4kib: Pe4Kib,
    pte_not_present: PteNotPresent,
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

