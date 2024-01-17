pub mod paging;
pub mod segment;

pub use paging::Paging;

pub const KIB: usize = 1 << 10;
pub const PAGE_SIZE: usize = 4 * KIB;

