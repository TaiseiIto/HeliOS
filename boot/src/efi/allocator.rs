//! # A memory allocator

use {
    alloc::alloc::Layout,
    core::alloc::GlobalAlloc,
    super::{
        SystemTable,
        Void,
    },
};

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator;

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align: usize = layout.align();
        let size: usize = layout.size();
        // [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)
        // 7.2 Memory Allocation Services
        // EFI_BOOT_SERVICES.AllocatePool()
        // "All allocations are eight-byte aligned."
        assert!(align <= 8);
        SystemTable::get()
            .allocate_pool(size)
            .map(|pointer| pointer.into())
            .unwrap()
    }

    unsafe fn dealloc(&self, pointer: *mut u8, _: Layout) {
        SystemTable::get()
            .free_pool(pointer.into())
            .unwrap()
    }
}

