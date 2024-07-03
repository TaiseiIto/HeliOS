pub mod frame;
pub mod page;
pub mod paging;

pub use {
    alloc::alloc::Layout,
    core::alloc::GlobalAlloc,
    crate::efi::SystemTable,
    frame::Frame,
    paging::Paging,
};

pub const KIB: usize = 1 << 10;

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator;

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align: usize = layout.align();
        let size: usize = layout.size();
        if (1..=8).contains(&align) {
            // [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)
            // 7.2 Memory Allocation Services
            // EFI_BOOT_SERVICES.AllocatePool()
            // "All allocations are eight-byte aligned."
            SystemTable::get().allocate_pool(size)
        } else if (1..=page::SIZE).contains(&align) {
            let pages: usize = (size + page::SIZE - 1) / page::SIZE;
            SystemTable::get().allocate_pages(pages)
        } else {
            panic!("Can't allocate memory.")
        }.map(|pointer| pointer.into()).unwrap()
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        let align: usize = layout.align();
        let size: usize = layout.size();
        if (1..=8).contains(&align) {
            // [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)
            // 7.2 Memory Allocation Services
            // EFI_BOOT_SERVICES.AllocatePool()
            // "All allocations are eight-byte aligned."
            assert!(align <= 8);
            SystemTable::get().free_pool(pointer.into())
        } else if (1..=page::SIZE).contains(&align) {
            let pages: usize = size / page::SIZE;
            SystemTable::get().free_pages(pointer.into(), pages)
        } else {
            panic!("Can't deallocate memory.")
        }.unwrap()
    }
}

