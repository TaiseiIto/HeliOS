//! # ELF Section
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

pub mod symbol;

use {
    alloc::collections::BTreeMap,
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
    pub fn bytes_in_file<'a>(&'a self, elf: &'a [u8]) -> &'a [u8] {
        let start: usize = self.sh_offset as usize;
        let end: usize = start + self.sh_size as usize;
        &elf[start..end]
    }

    pub fn sh_name(&self) -> Word {
        self.sh_name
    }

    pub fn string_table<'a>(&'a self, section: &'a [u8]) -> Option<BTreeMap</* Offset, in bytes, relative to the start of the string table section */ usize, /* String */ &'a str>> {
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
                    .ok())
                .collect())
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u32)]
enum Sht {
    #[allow(dead_code)]
    Null = 0x0,
    #[allow(dead_code)]
    Progbits = 0x1,
    #[allow(dead_code)]
    Symtab = 0x2,
    #[allow(dead_code)]
    Strtab = 0x3,
    #[allow(dead_code)]
    Rela = 0x4,
    #[allow(dead_code)]
    Hash = 0x5,
    #[allow(dead_code)]
    Dynamic = 0x6,
    #[allow(dead_code)]
    Note = 0x7,
    #[allow(dead_code)]
    Nobits = 0x8,
    #[allow(dead_code)]
    Rel = 0x9,
    #[allow(dead_code)]
    Shlib = 0x0A,
    #[allow(dead_code)]
    Dynsym = 0x0B,
    #[allow(dead_code)]
    InitArray = 0x0E,
    #[allow(dead_code)]
    FiniArray = 0x0F,
    #[allow(dead_code)]
    PreinitArray = 0x10,
    #[allow(dead_code)]
    Group = 0x11,
    #[allow(dead_code)]
    SymtabShndx = 0x12,
    #[allow(dead_code)]
    Num = 0x13,
    #[allow(dead_code)]
    LoOs = 0x60000000,
    #[allow(dead_code)]
    HiOs = 0x6fffffff,
    #[allow(dead_code)]
    LoProc = 0x70000000,
    #[allow(dead_code)]
    HiProc = 0x7fffffff,
}

#[bitfield(u64)]
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Flags {
    write: bool,
    alloc: bool,
    execinstr: bool,
    __: bool,
    merge: bool,
    strings: bool,
    info_link: bool,
    link_order: bool,
    os_nonconforming: bool,
    group: bool,
    tls: bool,
    #[bits(9)]
    __: u16,
    maskos: u8,
    #[bits(4)]
    maskproc: u8,
    __: u32,
}

struct Name<'a>(&'a [u8]);

impl fmt::Debug for Name<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#x?}", str::from_utf8(self.0))
    }
}

