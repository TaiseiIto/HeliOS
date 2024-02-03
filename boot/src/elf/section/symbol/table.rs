//! # ELF Section
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use {
    core::{
        mem,
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
        let len: usize = bytes.len() / mem::size_of::<Entry>();
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
    st_info: UnsignedChar,
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

