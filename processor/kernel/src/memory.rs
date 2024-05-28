pub mod page;

use {
    alloc::alloc::Layout,
    core::{
        alloc::GlobalAlloc,
        cell::UnsafeCell,
    },
    crate::Argument,
};

pub const KIB: usize = 1 << 10;

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator;

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unimplemented!()
    }

    unsafe fn dealloc(&self, address: *mut u8, _: Layout) {
        unimplemented!()
    }
}

