pub mod page;
pub mod paging;
pub mod segment;

pub use page::ContinuousPages;
pub use paging::Paging;

pub const KIB: usize = 1 << 10;

