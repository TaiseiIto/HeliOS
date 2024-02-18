pub mod page;
pub mod paging;
pub mod segment;

pub use page::Page;
pub use page::ContinuousPages;
pub use paging::Paging;

use {
    alloc::vec::Vec,
    core::fmt,
};

pub const KIB: usize = 1 << 10;

