//! # A memory allocator

extern crate alloc;

use {
    alloc::alloc::Layout,
    core::alloc::GlobalAlloc,
    crate::efi,
};

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator();

#[derive(Default)]
struct Allocator();

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align: usize = layout.align();
        let size: usize = layout.size();
        // [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)
        // 7.2 Memory Allocation Services
        // EFI_BOOT_SERVICES.AllocatePool()
        // "All allocations are eight-byte aligned."
        assert!(align <= 8);
        efi::SystemTable::get()
            .allocate(size)
            .map(|ptr| {
                let ptr: *const efi::Void = ptr as *const efi::Void;
                let ptr: usize = ptr as usize;
                ptr as *mut u8
            })
            .unwrap()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let ptr: usize = ptr as usize;
        let ptr: *const efi::Void = ptr as *const efi::Void;
        let ptr: &efi::Void = &*ptr;
        efi::SystemTable::get()
            .deallocate(ptr)
            .unwrap()
    }
}

