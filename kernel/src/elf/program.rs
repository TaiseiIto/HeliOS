//! # ELF Program
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use {
    super::{Addr, Off, Xword},
    crate::memory,
    alloc::{collections::BTreeMap, vec::Vec},
    bitfield_struct::bitfield,
    core::{cmp, ops::Range, slice},
};

/// # ELF Program Header
/// ## References
/// * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
/// * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    p_type: Pt,
    p_flags: Pf,
    p_offset: Off,
    p_vaddr: Addr,
    p_paddr: Addr,
    p_filesz: Xword,
    p_memsz: Xword,
    p_align: Xword,
}

impl Header {
    pub fn deploy(&self, elf: &[u8], pages: &mut [memory::Page]) {
        let start: usize = self.p_offset as usize;
        let end: usize = start + self.p_filesz as usize;
        let source_range: Range<usize> = start..end;
        let vaddr_range_in_bytes: Range<usize> = self.vaddr_range_in_bytes();
        let vaddr_range_in_pages: Range<usize> = self.vaddr_range_in_pages();
        vaddr_range_in_pages
            .step_by(memory::page::SIZE)
            .for_each(|start| {
                let page_range: Range<usize> = start..start + memory::page::SIZE;
                let vaddr_range: Range<usize> =
                    cmp::max(page_range.start, vaddr_range_in_bytes.start)
                        ..cmp::min(page_range.end, vaddr_range_in_bytes.end);
                let page: &mut memory::Page = pages
                    .iter_mut()
                    .find(|page| page.vaddr_range().contains(&vaddr_range.start))
                    .unwrap();
                let paddr_range: Range<usize> = page.vaddr2paddr(vaddr_range.start)
                    ..if page.vaddr_range().contains(&vaddr_range.end) {
                        page.vaddr2paddr(vaddr_range.end)
                    } else {
                        page.paddr_range().end
                    };
                let source_range_start: usize =
                    source_range.start + vaddr_range.start - vaddr_range_in_bytes.start;
                let source_range_end: usize =
                    cmp::min(source_range_start + vaddr_range.len(), source_range.end);
                let source_range: Range<usize> = source_range_start..source_range_end;
                let source: &[u8] = &elf[source_range];
                let destination: *mut u8 = paddr_range.start as *mut u8;
                let destination: &mut [u8] =
                    unsafe { slice::from_raw_parts_mut(destination, paddr_range.len()) };
                destination[0..source.len()].copy_from_slice(source);
            });
    }

    pub fn is_loadable_segment(&self) -> bool {
        matches!(self.p_type, Pt::Load)
    }

    pub fn is_writable(&self) -> bool {
        self.p_flags.w()
    }

    pub fn pages(&self) -> Vec<usize> {
        self.vaddr_range_in_pages()
            .filter(|vaddr| vaddr % memory::page::SIZE == 0)
            .collect()
    }

    pub fn set_page(&self, paging: &mut memory::Paging, vaddr2paddr: BTreeMap<usize, usize>) {
        let present: bool = true;
        let writable: bool = self.p_flags.w();
        let executable: bool = self.p_flags.x();
        self.pages().into_iter().for_each(|vaddr| {
            let paddr: usize = *vaddr2paddr.get(&vaddr).unwrap();
            paging.set_page(vaddr, paddr, present, writable, executable);
        });
    }

    fn vaddr_range_in_bytes(&self) -> Range<usize> {
        let start: usize = self.p_vaddr as usize;
        let end: usize = start + self.p_memsz as usize;
        start..end
    }

    fn vaddr_range_in_pages(&self) -> Range<usize> {
        let Range::<usize> { start, end } = self.vaddr_range_in_bytes();
        let start = (start / memory::page::SIZE) * memory::page::SIZE;
        let end = ((end + memory::page::SIZE - 1) / memory::page::SIZE) * memory::page::SIZE;
        start..end
    }
}

#[derive(Debug)]
#[repr(u32)]
enum Pt {
    #[allow(dead_code)]
    Null = 0,
    #[allow(dead_code)]
    Load = 1,
    #[allow(dead_code)]
    Dynamic = 2,
    #[allow(dead_code)]
    Interp = 3,
    #[allow(dead_code)]
    Note = 4,
    #[allow(dead_code)]
    Shlib = 5,
    #[allow(dead_code)]
    Phdr = 6,
    #[allow(dead_code)]
    LoOs = 0x60000000,
    #[allow(dead_code)]
    HiOs = 0x6fffffff,
    #[allow(dead_code)]
    LoProc = 0x70000000,
    #[allow(dead_code)]
    HiProc = 0x7fffffff,
}

#[bitfield(u32)]
struct Pf {
    x: bool,
    w: bool,
    r: bool,
    #[bits(13)]
    __: u16,
    maskos: u8,
    mascproc: u8,
}
