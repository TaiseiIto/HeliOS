use {
    core::{
        fmt,
        ops::Range,
        slice,
    },
    crate::{
        efi,
        memory,
    },
};

pub struct Loader {
    program_address_range: Range<usize>,
    stack_floor: usize,
}

impl Loader {
    pub fn new(binary: &[u8], physical_range: Range<efi::memory::PhysicalAddress>) -> Self {
        let base: usize = physical_range.start as usize;
        let stack_floor: usize = physical_range.end as usize;
        let program_address_range: Range<usize> = base..base + binary.len();
        let physical_start: *mut u8 = physical_range.start as *mut u8;
        unsafe { slice::from_raw_parts_mut(physical_start, binary.len()) }
            .copy_from_slice(binary);
        Self {
            program_address_range,
            stack_floor,
        }
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

