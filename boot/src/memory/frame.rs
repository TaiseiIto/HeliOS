use {super::page, core::fmt};

#[repr(align(4096))]
pub struct Frame {
    bytes: [u8; page::SIZE],
}

impl Frame {
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
        let bytes: [u8; page::SIZE] = [u8::default(); page::SIZE];
        Self { bytes }
    }
}
