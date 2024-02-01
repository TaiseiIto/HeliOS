//! # ELF file
//! ## References
//! * [Tool Interface Standard (TIS) Executable and Linking Format (ELF) Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf)

use {
    alloc::vec::Vec,
    core::fmt,
};

/// # ELF file
/// ## References
/// * [Tool Interface Standard (TIS) Executable and Linking Format (ELF) Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf)
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
}

impl fmt::Debug for File {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("File")
            .field("header", self.header())
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

type Addr = u32;
type Half = u16;
type Off = u32;
type Sword = i32;
type Word = u32;
type UnsignedChar = u8;

const EI_NIDENT: usize = 16;

#[derive(Debug)]
#[repr(C)]
struct Header {
    e_ident: [UnsignedChar; EI_NIDENT],
    e_type: Et,
    e_machine: Half,
    e_version: Word,
    e_entry: Addr,
    e_phoff: Off,
    e_shoff: Off,
    e_flags: Word,
    e_ehsize: Half,
    e_phentsize: Half,
    e_phnum: Half,
    e_shentsize: Half,
    e_shnum: Half,
    e_shstrndx: Half,
}

#[derive(Debug)]
#[repr(u16)]
enum Et {
    None = 0,
    Rel = 1,
    Exec = 2,
    Dyn = 3,
    Core = 4,
    Loproc = 0xff00,
    Hiproc = 0xffff,
}

