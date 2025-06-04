use {
    alloc::vec::Vec,
    core::{
        cmp,
        fmt,
        ops,
        ptr,
    },
    crate::{
        com2_println,
        efi,
        memory,
    },
};

pub mod real_mode;

pub struct Loader {
    program_address_range: ops::Range<usize>,
    stack_address_range: ops::Range<usize>,
}

impl Loader {
    pub fn allocate_pages() -> ops::Range<efi::memory::PhysicalAddress> {
        let range: ops::Range<usize> = efi::SystemTable::get()
            .memory_map()
            .unwrap()
            .iter()
            .filter(|descriptor| descriptor.is_available())
            .map(|descriptor| descriptor.physical_address_range())
            .filter_map(|descriptor_range| {
                let ops::Range {
                    start: descriptor_start,
                    end: descriptor_end,
                } = descriptor_range;
                let descriptor_start: usize = descriptor_start as usize;
                let descriptor_end: usize = descriptor_end as usize;
                let ops::Range {
                    start: real_start,
                    end: real_end,
                } = real_mode::memory::address::RANGE;
                let start: usize = cmp::max(descriptor_start, real_start);
                let end: usize = cmp::min(descriptor_end, real_end);
                let loader_range: ops::Range<usize> = start..end;
                (!loader_range.is_empty()).then_some(loader_range)
            })
            .max_by(|x, y| (x.end - x.start).cmp(&(y.end - y.start)))
            .unwrap();
        com2_println!("range = {:#x?}", range);
        let pages: usize = (range.end - range.start) / memory::page::SIZE;
        efi::SystemTable::get()
            .allocate_specific_pages(range.start, pages)
            .unwrap()
    }

    pub fn new(binary: &[u8], physical_range: ops::Range<efi::memory::PhysicalAddress>) -> Self {
        let program_start: usize = physical_range.start as usize;
        let program_size: usize = binary.len();
        let program_end: usize = program_start + program_size;
        let program_address_range: ops::Range<usize> = program_start..program_end;
        binary
            .iter()
            .cloned()
            .zip(program_address_range
                .clone()
                .map(|program_address| program_address as *mut u8))
            .for_each(|(source, destination)| unsafe {
                ptr::write_volatile(destination, source);
            });
        let stack_ceil: usize = program_end;
        let stack_floor: usize = physical_range.end as usize;
        let stack_address_range: ops::Range<usize> = stack_ceil..stack_floor;
        stack_address_range
            .clone()
            .map(|stack_address| stack_address as *mut u8)
            .for_each(|stack_address| unsafe {
                ptr::write_volatile(stack_address, 0);
            });
        com2_println!("processor::boot::Loader program_address_range = {:#x?}", program_address_range);
        com2_println!("processor::boot::Loader stack_address_range = {:#x?}", stack_address_range);
        Self {
            program_address_range,
            stack_address_range,
        }
    }

    pub fn program(&self) -> Vec<u8> {
        self.program_address_range
            .clone()
            .map(|program_address| program_address as *const u8)
            .map(|program_address| unsafe {
                ptr::read_volatile(program_address)
            })
            .collect()
    }

    pub fn stack(&self) -> Vec<u8> {
        self.stack_address_range
            .clone()
            .map(|stack_address| stack_address as *const u8)
            .map(|stack_address| unsafe {
                ptr::read_volatile(stack_address)
            })
            .collect()
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

