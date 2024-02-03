//! # ELF Section
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
        fmt,
        iter,
        str,
    },
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
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
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

impl Header {
    pub fn bytes<'a>(&'a self, elf: &'a [u8]) -> &[u8] {
        let begin: usize = self.sh_offset as usize;
        let end: usize = begin + self.sh_size as usize;
        &elf[begin..end]
    }

    pub fn sh_name(&self) -> Word {
        self.sh_name
    }

    pub fn string_table<'a>(&'a self, section: &'a [u8]) -> Option<impl Iterator<Item = (/* Offset, in bytes, relative to the start of the string table section */ usize, /* String */ &'a str)>> {
        (self.sh_type == Sht::Strtab)
            .then(|| iter::once(0)
                .chain(section
                    .iter()
                    .enumerate()
                    .filter(|(_index, byte)| **byte == 0)
                    .map(|(index, _byte)| index + 1))
                .zip(section
                    .split(|byte| *byte == 0)
                    .map(|bytes| str::from_utf8(bytes)))
                .filter_map(|(index, string)| string
                    .map(|string| (index, string))
                    .ok()))
    }
}

#[allow(dead_code)]
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
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
#[derive(Eq, Ord, PartialEq, PartialOrd)]
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

struct Name<'a>(&'a [u8]);

impl fmt::Debug for Name<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#x?}", str::from_utf8(self.0))
    }
}

