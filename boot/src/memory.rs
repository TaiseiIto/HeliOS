pub mod frame;
pub mod page;
pub mod paging;
pub mod segment;

pub use {
    frame::Frame,
    paging::Paging,
};

use core::fmt;

pub const KIB: usize = 1 << 10;

