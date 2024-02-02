//! # ELF Section
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use {
    bitfield_struct::bitfield,
    super::{
        Addr,
        Off,
        Word,
        Xword,
    },
};

/// # ELF Section Header
/// ## References
/// * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
/// * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    sh_name: Word,
    sh_type: Sht,
    sh_flags: Flags,
    sh_addr: Addr,
    sh_offset: Off,
    sh_size: Xword,
    sh_link: Word,
    sh_info: Word,
    sh_addralign: Xword,
    sh_entsize: Xword,
}

#[allow(dead_code)]
#[derive(Debug)]
#[repr(u32)]
enum Sht {
    Null = 0x0,
    Progbits = 0x1,
    Symtab = 0x2,
    Strtab = 0x3,
    Rela = 0x4,
    Hash = 0x5,
    Dynamic = 0x6,
    Note = 0x7,
    Nobits = 0x8,
    Rel = 0x9,
    Shlib = 0x0A,
    Dynsym = 0x0B,
    InitArray = 0x0E,
    FiniArray = 0x0F,
    PreinitArray = 0x10,
    Group = 0x11,
    SymtabShndx = 0x12,
    Num = 0x13,
    LoOs = 0x60000000,
    HiOs = 0x6fffffff,
    LoProc = 0x70000000,
    HiProc = 0x7fffffff,
}

#[bitfield(u64)]
struct Flags {
    write: bool,
    alloc: bool,
    execinstr: bool,
    #[bits(access = RO)]
    reserved0: bool,
    merge: bool,
    strings: bool,
    info_link: bool,
    link_order: bool,
    os_nonconforming: bool,
    group: bool,
    tls: bool,
    #[bits(9, access = RO)]
    reserved1: u16,
    maskos: u8,
    #[bits(4)]
    maskproc: u8,
    reserved2: u32,
}

