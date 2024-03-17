use {
    alloc::vec::Vec,
    core::{
        ops::Range,
        slice,
    },
    crate::{
        efi,
        memory,
    },
};

#[derive(Debug)]
pub struct Loader {
    base: usize,
    stack_floor: usize,
}

impl Loader {
    pub fn new(binary: &[u8], base: usize, stack_floor: usize) -> Self {
        let pages: usize = (stack_floor - base) / memory::page::SIZE;
        let physical_range: Range<efi::memory::PhysicalAddress> = efi::SystemTable::get()
            .allocate_specific_pages(base, pages)
            .unwrap();
        let physical_start: *mut u8 = physical_range.start as *mut u8;
        let length: usize = (physical_range.end - physical_range.start) as usize;
        unsafe { slice::from_raw_parts_mut(physical_start, length) }
            .copy_from_slice(binary);
        Self {
            base,
            stack_floor,
        }
    }
}

