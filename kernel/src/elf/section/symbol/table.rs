//! # ELF Section
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use {
    super::super::super::{Addr, Half, UnsignedChar, Word, Xword},
    bitfield_struct::bitfield,
    core::{mem::size_of, slice},
};

/// # ELF Symbol Table
/// ## References
/// * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
/// * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
#[derive(Debug)]
pub struct Table<'a>(&'a [Entry]);

impl<'a> IntoIterator for Table<'a> {
    type Item = &'a Entry;
    type IntoIter = slice::Iter<'a, Entry>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> From<&'a [u8]> for Table<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        let len: usize = bytes.len() / size_of::<Entry>();
        let entry: &u8 = bytes.first().unwrap();
        let entry: *const u8 = entry as *const u8;
        let entry: *const Entry = entry as *const Entry;
        let entries: &[Entry] = unsafe { slice::from_raw_parts(entry, len) };
        Self(entries)
    }
}

/// # ELF Symbol Table Entry
/// ## References
/// * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
/// * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
#[derive(Debug)]
#[repr(C)]
pub struct Entry {
    st_name: Word,
    st_info: Info,
    st_other: UnsignedChar,
    st_shndx: Half,
    st_value: Addr,
    st_size: Xword,
}

impl Entry {
    pub fn st_name(&self) -> Word {
        self.st_name
    }
}

#[bitfield(u8)]
struct Info {
    #[bits(4)]
    symbol_type: Stt,
    #[bits(4)]
    binding_attributes: Stb,
}

#[derive(Debug)]
#[repr(u8)]
enum Stt {
    Notype = 0,
    Object = 1,
    Func = 2,
    Section = 3,
    File = 4,
    Common = 5,
    Tls = 6,
    Other7 = 7,
    Other8 = 8,
    Other9 = 9,
    Os10 = 10,
    Os11 = 11,
    Os12 = 12,
    Proc13 = 13,
    Proc14 = 14,
    Proc15 = 15,
}

impl Stt {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Notype,
            1 => Self::Object,
            2 => Self::Func,
            3 => Self::Section,
            4 => Self::File,
            5 => Self::Common,
            6 => Self::Tls,
            7 => Self::Other7,
            8 => Self::Other8,
            9 => Self::Other9,
            10 => Self::Os10,
            11 => Self::Os11,
            12 => Self::Os12,
            13 => Self::Proc13,
            14 => Self::Proc14,
            15 => Self::Proc15,
            _ => panic!("Invalid Stt!"),
        }
    }

    const fn into_bits(self) -> u8 {
        match self {
            Self::Notype => 0,
            Self::Object => 1,
            Self::Func => 2,
            Self::Section => 3,
            Self::File => 4,
            Self::Common => 5,
            Self::Tls => 6,
            Self::Other7 => 7,
            Self::Other8 => 8,
            Self::Other9 => 9,
            Self::Os10 => 10,
            Self::Os11 => 11,
            Self::Os12 => 12,
            Self::Proc13 => 13,
            Self::Proc14 => 14,
            Self::Proc15 => 15,
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
enum Stb {
    Local = 0,
    Global = 1,
    Weak = 2,
    Other3 = 3,
    Other4 = 4,
    Other5 = 5,
    Other6 = 6,
    Other7 = 7,
    Other8 = 8,
    Other9 = 9,
    Os10 = 10,
    Os11 = 11,
    Os12 = 12,
    Proc13 = 13,
    Proc14 = 14,
    Proc15 = 15,
}

impl Stb {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Local,
            1 => Self::Global,
            2 => Self::Weak,
            3 => Self::Other3,
            4 => Self::Other4,
            5 => Self::Other5,
            6 => Self::Other6,
            7 => Self::Other7,
            8 => Self::Other8,
            9 => Self::Other9,
            10 => Self::Os10,
            11 => Self::Os11,
            12 => Self::Os12,
            13 => Self::Proc13,
            14 => Self::Proc14,
            15 => Self::Proc15,
            _ => panic!("Invalid Stb!"),
        }
    }

    const fn into_bits(self) -> u8 {
        match self {
            Self::Local => 0,
            Self::Global => 1,
            Self::Weak => 2,
            Self::Other3 => 3,
            Self::Other4 => 4,
            Self::Other5 => 5,
            Self::Other6 => 6,
            Self::Other7 => 7,
            Self::Other8 => 8,
            Self::Other9 => 9,
            Self::Os10 => 10,
            Self::Os11 => 11,
            Self::Os12 => 12,
            Self::Proc13 => 13,
            Self::Proc14 => 14,
            Self::Proc15 => 15,
        }
    }
}
