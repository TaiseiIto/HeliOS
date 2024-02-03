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

    fn shstrtab(&self) -> BTreeMap</* Offset, in bytes, relative to the start of the string table section */ usize, /* String */ &str> {
        let section_header2offset2string: BTreeMap<&section::Header, BTreeMap<usize, &str>> = self
            .string_tables()
            .map(|(section_header, offset2string)| (section_header, offset2string.collect()))
            .collect();
        let shstrtab_section_sh_name: Word = *section_header2offset2string
            .values()
            .find_map(|offset2string| offset2string
                .iter()
                .find_map(|(offset, string)| (*string == ".shstrtab")
                    .then_some(offset)))
            .unwrap() as Word;
        let shstrtab_section_header: &section::Header = section_header2offset2string
            .keys()
            .find(|section_header| section_header.sh_name() == shstrtab_section_sh_name)
            .unwrap();
        section_header2offset2string[shstrtab_section_header].clone()
    }

    fn string_tables(&self) -> impl Iterator<Item = (&section::Header, impl Iterator<Item = (/* Offset, in bytes, relative to the start of the string table section */ usize, /* String */ &str)>)> {
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
        let shstrtab: BTreeMap<usize, &str> = self.shstrtab();
        formatter
            .debug_struct("File")
            .field("header", header)
            .field("section_headers", &section_headers)
            .field("shstrtab", &shstrtab)
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

