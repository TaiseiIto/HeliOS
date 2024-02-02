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
    core::fmt,
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

    fn sections(&self) -> BTreeMap<&section::Header, &[u8]> {
        self
            .header()
            .section_headers()
            .map(|section_header| (section_header, section_header.bytes(&self.bytes)))
            .collect()
    }
}

impl fmt::Debug for File {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: &Header = self.header();
        let sections: BTreeMap<&section::Header, &[u8]> = self.sections();
        formatter
            .debug_struct("File")
            .field("header", header)
            .field("sections", &sections)
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

