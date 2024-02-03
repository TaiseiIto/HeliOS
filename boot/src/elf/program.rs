//! # ELF Program
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use super::{
    Addr,
    Off,
    Word,
    Xword,
};

/// # ELF Program Header
/// ## References
/// * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
/// * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    p_type: Pt,
    p_flags: Word,
    p_offset: Off,
    p_vaddr: Addr,
    p_paddr: Addr,
    p_filesz: Xword,
    p_memsz: Xword,
    p_align: Xword,
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

