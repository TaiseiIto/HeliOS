pub mod paging;
pub mod segment;

pub use paging::Paging;

use {
    alloc::vec::Vec,
    core::fmt,
};

pub const KIB: usize = 1 << 10;
pub const PAGE_SIZE: usize = 4 * KIB;

pub struct Pages {
    pages: Vec<Page>,
    paddr: usize,
    vaddr: usize,
}

#[repr(align(4096))]
pub struct Page {
    bytes: [u8; PAGE_SIZE],
}

