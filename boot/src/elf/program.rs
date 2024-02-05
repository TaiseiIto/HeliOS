//! # ELF Program
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::ops::Range,
    crate::memory,
    super::{
        Addr,
        Off,
        Xword,
    },
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
    pub fn bytes_in_file<'a>(&'a self, elf: &'a [u8]) -> &'a [u8] {
        let begin: usize = self.p_offset as usize;
        let end: usize = begin + self.p_filesz as usize;
        &elf[begin..end]
    }

    pub fn pages(&self) -> Vec<usize> {
        self.vaddr_range_in_pages()
            .filter(|vaddr| vaddr % memory::PAGE_SIZE == 0)
            .collect()
    }

    fn vaddr_range_in_bytes(&self) -> Range<usize> {
        let begin: usize = self.p_vaddr as usize;
        let end: usize = begin + self.p_memsz as usize;
        begin..end
    }

    fn vaddr_range_in_pages(&self) -> Range<usize> {
        let Range::<usize> {
            start,
            end,
        } = self.vaddr_range_in_bytes();
        let start = (start / memory::PAGE_SIZE) * memory::PAGE_SIZE;
        let end = ((end + memory::PAGE_SIZE - 1) / memory::PAGE_SIZE) * memory::PAGE_SIZE;
        start..end
    }
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u32)]
enum Pt {
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interp = 3,
    Note = 4,
    Shlib = 5,
    Phdr = 6,
    LoOs = 0x60000000,
    HiOs = 0x6fffffff,
    LoProc = 0x70000000,
    HiProc = 0x7fffffff,
}

#[bitfield(u32)]
struct Pf {
    x: bool,
    w: bool,
    r: bool,
    #[bits(13, access = RO)]
    reserved0: u16,
    maskos: u8,
    mascproc: u8,
}

