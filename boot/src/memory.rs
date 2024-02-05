pub mod paging;
pub mod segment;

pub use paging::Paging;

use core::fmt;

pub const KIB: usize = 1 << 10;
pub const PAGE_SIZE: usize = 4 * KIB;

#[repr(align(4096))]
pub struct Frame {
    bytes: [u8; PAGE_SIZE],
}

impl Frame {
    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    pub fn paddr(&self) -> usize {
        let physical_address: &u8 = &self.bytes[0];
        let physical_address: *const u8 = physical_address as *const u8;
        physical_address as usize
    }
}

impl fmt::Debug for Frame {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Frame")
            .field("paddr", &self.paddr())
            .finish()
    }
}

impl Default for Frame {
    fn default() -> Self {
        let bytes: [u8; PAGE_SIZE] = [u8::default(); PAGE_SIZE];
        Self {
            bytes,
        }
    }
}

