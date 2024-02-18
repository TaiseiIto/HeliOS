pub mod frame;
pub mod page;
pub mod paging;
pub mod segment;

pub use {
    frame::Frame,
    paging::Paging,
};

pub const KIB: usize = 1 << 10;

