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

impl fmt::Debug for File {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("File")
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

