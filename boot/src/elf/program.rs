//! # ELF Program
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use {
    alloc::{
        collections::BTreeMap,
        vec::Vec,
    },
    bitfield_struct::bitfield,
    core::{
        ops::Range,
        slice,
    },
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
    pub fn deploy(&self, elf: &[u8]) {
        let begin: usize = self.p_vaddr as usize;
        let begin: *mut u8 = begin as *mut u8;
        let length: usize = self.p_memsz as usize;
        let bytes_in_memory: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(begin, length)
        };
        let begin: usize = self.p_offset as usize;
        let end: usize = begin + self.p_filesz as usize;
        let bytes_in_file: &[u8] = &elf[begin..end];
        bytes_in_memory[0..bytes_in_file.len()].copy_from_slice(bytes_in_file);
    }

    pub fn pages(&self) -> Vec<usize> {
        self.vaddr_range_in_pages()
            .filter(|vaddr| vaddr % memory::PAGE_SIZE == 0)
            .collect()
    }

    pub fn set_page(&self, paging: &mut memory::Paging, vaddr2paddr: BTreeMap<usize, usize>) {
        let present: bool = true;
        let writable: bool = self.p_flags.w();
        let executable: bool = self.p_flags.x();
        self.pages()
            .into_iter()
            .for_each(|vaddr| {
                let paddr: usize = *vaddr2paddr
                    .get(&vaddr)
                    .unwrap();
                paging.set_page(vaddr, paddr, present, writable, executable);
            });
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

