//! # A memory allocator

use {
    alloc::alloc::Layout,
    core::alloc::GlobalAlloc,
    crate::memory,
    super::SystemTable,
};

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator;

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align: usize = layout.align();
        assert!(align.is_power_of_two());
        let size: usize = layout.size();
        match align {
            memory::PAGE_SIZE => {
                let pages: usize = size / memory::PAGE_SIZE;
                SystemTable::get().allocate_pages(pages)
            },
            align => {
                // [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)
                // 7.2 Memory Allocation Services
                // EFI_BOOT_SERVICES.AllocatePool()
                // "All allocations are eight-byte aligned."
                assert!(align <= 8);
                SystemTable::get().allocate_pool(size)
            },
        }.map(|pointer| pointer.into()).unwrap()
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let align: usize = layout.align();
        assert!(align.is_power_of_two());
        let size: usize = layout.size();
        match align {
            memory::PAGE_SIZE => {
                let pages: usize = size / memory::PAGE_SIZE;
                SystemTable::get().free_pages(pointer.into(), pages)
            },
            align => {
                // [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)
                // 7.2 Memory Allocation Services
                // EFI_BOOT_SERVICES.AllocatePool()
                // "All allocations are eight-byte aligned."
                assert!(align <= 8);
                SystemTable::get().free_pool(pointer.into())
            },
        }.unwrap()
    }
}

