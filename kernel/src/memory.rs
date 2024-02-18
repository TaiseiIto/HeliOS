pub mod page;
pub mod paging;
pub mod segment;
pub mod stack;

pub use {
    page::ContinuousPages,
    paging::Paging,
    stack::Stack,
};

pub const KIB: usize = 1 << 10;

