use core::{
    fmt,
    ops::Range,
    slice,
};

pub struct Loader {
    program_address_range: Range<usize>,
    stack_floor: usize,
}

impl Loader {
    pub fn entry_point(&self) -> usize {
        self.program_address_range.start
    }

    pub fn program(&self) -> &[u8] {
        let start: *const u8 = self.program_address_range.start as *const u8;
        let length: usize = self.program_address_range.end - self.program_address_range.start;
        unsafe {
            slice::from_raw_parts(start, length)
        }
    }
}

impl fmt::Debug for Loader {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Loader")
            .field("program", &self.program())
            .field("stack_floor", &self.stack_floor)
            .finish()
    }
}

