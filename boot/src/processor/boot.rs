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
    stack_address_range: Range<usize>,
}

impl Loader {
    pub fn allocate_pages(base: usize, stack_floor: usize) -> Range<efi::memory::PhysicalAddress> {
        let processor_boot_loader_pages: usize = (stack_floor - base) / memory::page::SIZE;
        efi::SystemTable::get()
            .allocate_specific_pages(base, processor_boot_loader_pages)
            .unwrap()
    }

    pub fn new(binary: &[u8], physical_range: Range<efi::memory::PhysicalAddress>) -> Self {
        let program_start: usize = physical_range.start as usize;
        let program_size: usize = binary.len();
        let program_end: usize = program_start + program_size;
        let program_address_range: Range<usize> = program_start..program_end;
        let program_destination: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(program_start as *mut u8, program_size)
        };
        program_destination.copy_from_slice(binary);
        let stack_ceil: usize = program_end;
        let stack_floor: usize = physical_range.end as usize;
        let stack_size: usize = stack_floor - stack_ceil;
        let stack_address_range: Range<usize> = stack_ceil..stack_floor;
        let stack_destination: &mut [u8] = unsafe {
            slice::from_raw_parts_mut(stack_ceil as *mut u8, stack_size)
        };
        stack_destination
            .iter_mut()
            .for_each(|byte| *byte = 0);
        Self {
            program_address_range,
            stack_address_range,
        }
    }

    pub fn program(&self) -> &[u8] {
        let start: *const u8 = self.program_address_range.start as *const u8;
        let length: usize = self.program_address_range.end - self.program_address_range.start;
        unsafe {
            slice::from_raw_parts(start, length)
        }
    }

    pub fn stack(&self) -> &[u8] {
        let start: *const u8 = self.stack_address_range.start as *const u8;
        let length: usize = self.stack_address_range.end - self.stack_address_range.start;
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
            .field("stack", &self.stack())
            .finish()
    }
}

