//! # ELF Section
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use {
    bitfield_struct::bitfield,
    core::{
        mem::size_of,
        slice,
    },
    super::super::super::{
        Addr,
        Half,
        UnsignedChar,
        Word,
        Xword,
    },
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
        let entry: &u8 = bytes
            .first()
            .unwrap();
        let entry: *const u8 = entry as *const u8;
        let entry: *const Entry = entry as *const Entry;
        let entries: &[Entry] = unsafe {
            slice::from_raw_parts(entry, len)
        };
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
enum Stt{
    Notype = 0,
    Object = 1,
    Func = 2,
    Section = 3,
    File = 4,
    LoOs = 10,
    HiOs = 12,
    LoProc = 13,
    HiProc = 15,
}

impl Stt {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Notype ,
            1 => Self::Object ,
            2 => Self::Func ,
            3 => Self::Section ,
            4 => Self::File ,
            10 => Self::LoOs ,
            12 => Self::HiOs ,
            13 => Self::LoProc ,
            15 => Self::HiProc ,
            _ => panic!("Invalid Stt!"),
        }
    }

    const fn into_bits(self) -> u8 {
        match self {
            Stt::Notype  => 0,
            Stt::Object  => 1,
            Stt::Func  => 2,
            Stt::Section  => 3,
            Stt::File  => 4,
            Stt::LoOs  => 10,
            Stt::HiOs  => 12,
            Stt::LoProc  => 13,
            Stt::HiProc  => 15,
        }
    }
}

#[derive(Debug)]
#[repr(u8)]
enum Stb {
    Local = 0,
    Global = 1,
    Weak = 2,
    LoOs = 10,
    HiOs = 12,
    LoProc = 13,
    HiProc = 15,
}

impl Stb {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Local ,
            1 => Self::Global ,
            2 => Self::Weak ,
            10 => Self::LoOs ,
            12 => Self::HiOs ,
            13 => Self::LoProc ,
            15 => Self::HiProc ,
            _ => panic!("Invalid Stb!"),
        }
    }

    const fn into_bits(self) -> u8 {
        match self {
            Stb::Local  => 0,
            Stb::Global  => 1,
            Stb::Weak  => 2,
            Stb::LoOs  => 10,
            Stb::HiOs  => 12,
            Stb::LoProc  => 13,
            Stb::HiProc  => 15,
        }
    }
}

