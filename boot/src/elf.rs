//! # ELF file
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

mod header;
mod program;
mod section;

pub use section::symbol;

use {
    alloc::{
        boxed::Box,
        collections::{
            BTreeMap,
            BTreeSet,
        },
        vec::Vec,
    },
    core::{
        fmt,
        str,
    },
    crate::{
        com2_print,
        com2_println,
        memory,
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
    pub fn deploy(&self) {
        com2_println!("Deploy kernel.elf");
        let pages: BTreeSet<usize> = self.program_headers()
            .into_iter()
            .flat_map(|program_header| program_header
                .pages()
                .into_iter())
            .collect();
        let vaddr2frame: BTreeMap<usize, Box<memory::Frame>> = pages
            .into_iter()
            .map(|vaddr| (vaddr, Box::default()))
            .collect();
        com2_println!("vaddr2frame = {:#x?}", vaddr2frame);
    }

    fn header(&self) -> &Header {
        let header: &u8 = &self.bytes[0];
        let header: *const u8 = header as *const u8;
        let header: *const Header = header as *const Header;
        unsafe {
            &*header
        }
    }

    fn program_headers(&self) -> Vec<&program::Header> {
        self.header()
            .program_headers()
    }

    fn section_bytes<'a>(&'a self, section_header: &'a section::Header) -> &'a [u8] {
        section_header.bytes(&self.bytes)
    }

    fn section_header(&self, section_name: &str) -> Option<&section::Header> {
        let sh_name: Word = self
            .shstrtab()
            .into_iter()
            .find_map(|(offset, string)| (string == section_name)
                .then_some(offset))
            .unwrap() as Word;
        self.section_headers()
            .into_iter()
            .find(|section_header| section_header.sh_name() == sh_name)
    }

    fn section_headers(&self) -> Vec<&section::Header> {
        self.header()
            .section_headers()
    }

    fn section_header2bytes(&self) -> BTreeMap<&section::Header, &[u8]> {
        self.section_headers()
            .into_iter()
            .map(|section_header| (section_header, section_header.bytes(&self.bytes)))
            .collect()
    }

    fn section_name2section_header(&self) -> BTreeMap<&str, &section::Header> {
        let offset2string: BTreeMap<usize, &str> = self.shstrtab();
        self.section_headers()
            .into_iter()
            .filter_map(|section_header| offset2string
                .get(&(section_header.sh_name() as usize))
                .map(|section_name| (*section_name, section_header)))
            .collect()
    }

    fn shstrtab(&self) -> BTreeMap</* Offset, in bytes, relative to the start of the string table section */ usize, /* String */ &str> {
        let section_header2offset2string: BTreeMap<&section::Header, BTreeMap<usize, &str>> = self.string_tables();
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

    fn string_tables(&self) -> BTreeMap<&section::Header, BTreeMap</* Offset, in bytes, relative to the start of the string table section */ usize, /* String */ &str>> {
        self.section_header2bytes()
            .into_iter()
            .filter_map(|(section_header, bytes)| section_header
                .string_table(bytes)
                .map(|string_table| (section_header, string_table)))
            .collect()
    }

    fn strtab(&self) -> BTreeMap</* Offset, in bytes, relative to the start of the string table section */ usize, /* String */ &str> {
        let strtab_section_header: &section::Header = self
            .section_header(".strtab")
            .unwrap();
        let strtab_section_bytes: &[u8] = self.section_bytes(strtab_section_header);
        strtab_section_header
            .string_table(strtab_section_bytes)
            .unwrap()
    }

    fn symbol_name2symbol_entry(&self) -> BTreeMap<&str, &symbol::table::Entry> {
        let offset2string: BTreeMap<usize, &str> = self.strtab();
        self.symtab()
            .into_iter()
            .filter_map(|symbol_entry| offset2string
                .get(&(symbol_entry.st_name() as usize))
                .map(|symbol_name| (*symbol_name, symbol_entry)))
            .collect()
    }

    fn symtab(&self) -> symbol::Table {
        let symtab_section_header: &section::Header = self
            .section_header(".symtab")
            .unwrap();
        let symtab_section_bytes: &[u8] = self.section_bytes(symtab_section_header);
        symtab_section_bytes.into()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: &Header = self.header();
        let program_headers: Vec<&program::Header> = self.program_headers();
        let section_name2section_header: BTreeMap<&str, &section::Header> = self.section_name2section_header();
        let symbol_name2symbol_entry: BTreeMap<&str, &symbol::table::Entry> = self.symbol_name2symbol_entry();
        formatter
            .debug_struct("File")
            .field("header", header)
            .field("program_headers", &program_headers)
            .field("section_name2section_header", &section_name2section_header)
            .field("symbol_name2symbol_entry", &symbol_name2symbol_entry)
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
type Half = u16;
type Off = u64;
type UnsignedChar = u8;
type Word = u32;
type Xword = u64;

