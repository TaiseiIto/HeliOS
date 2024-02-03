//! # ELF file
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

mod header;
mod section;

use {
    alloc::{
        collections::BTreeMap,
        vec::Vec,
    },
    core::{
        fmt,
        str,
    },
    header::Header,
};

/// # ELF file
/// ## References
/// * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
/// * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
pub struct File {
    bytes: Vec<u8>,
}

impl File {
    fn header(&self) -> &Header {
        let header: &u8 = &self.bytes[0];
        let header: *const u8 = header as *const u8;
        let header: *const Header = header as *const Header;
        unsafe {
            &*header
        }
    }

    fn section_headers(&self) -> impl Iterator<Item = &section::Header> {
        self.header()
            .section_headers()
    }

    fn sections(&self) -> impl Iterator<Item = (&section::Header, &[u8])> {
        self.section_headers()
            .map(|section_header| (section_header, section_header.bytes(&self.bytes)))
    }

    fn string_tables(&self) -> impl Iterator<Item = (&section::Header, BTreeMap<usize, &str>)> {
        self.sections()
            .filter_map(|(section_header, bytes)| section_header
                .string_table(bytes)
                .map(|string_table| (section_header, string_table)))
    }
}

impl fmt::Debug for File {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: &Header = self.header();
        let section_headers: Vec<&section::Header> = self
            .section_headers()
            .collect();
        let string_tables: BTreeMap<&section::Header, BTreeMap<usize, &str>> = self
            .string_tables()
            .collect();
        formatter
            .debug_struct("File")
            .field("header", header)
            .field("section_headers", &section_headers)
            .field("string_tables", &string_tables)
            .finish()
    }
}

impl From<Vec<u8>> for File {
    fn from(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
        }
    }
}

type Addr = u64;
type Off = u64;
type Half = u16;
type Word = u32;
type Sword = i32;
type Xword = u64;
type Sxword = i64;
type UnsignedChar = u8;

