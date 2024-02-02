//! # ELF Section
//! ## References
//! * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
//! * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)

use super::{
    Addr,
    Off,
    Word,
    Xword,
};

/// # ELF Section Header
/// ## References
/// * [ELF-64 Object File Format](https://uclibc.org/docs/elf-64-gen.pdf)
/// * [Wikipedia Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
#[derive(Debug)]
#[repr(C)]
pub struct Header {
    sh_name: Word,
    sh_type: Word,
    sh_flags: Xword,
    sh_addr: Addr,
    sh_offset: Off,
    sh_size: Xword,
    sh_link: Word,
    sh_info: Word,
    sh_addralign: Xword,
    sh_entsize: Xword,
}

