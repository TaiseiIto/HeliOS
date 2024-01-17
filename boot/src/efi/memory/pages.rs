use {
    alloc::alloc::{
        alloc,
        dealloc,
        Layout,
    },
    crate::memory,
    super::VirtualAddress,
};

pub struct Pages {
    pointer: VirtualAddress,
    pages: usize,
}

impl Pages {
    pub fn new(pages: usize) -> Self {
        let size: usize = pages * memory::PAGE_SIZE;
        let align: usize = memory::PAGE_SIZE;
        let layout = Layout::from_size_align(size, align).unwrap();
        let pointer: VirtualAddress = unsafe {
            alloc(layout)
        } as VirtualAddress;
        Self {
            pointer,
            pages,
        }
    }
}

impl Drop for Pages {
    fn drop(&mut self) {
        let Self {
            pointer,
            pages,
        } = self;
        let pointer: *mut u8 = *pointer as *mut u8;
        let size: usize = *pages * memory::PAGE_SIZE;
        let align: usize = memory::PAGE_SIZE;
        let layout = Layout::from_size_align(size, align).unwrap();
        unsafe {
            dealloc(pointer, layout);
        }
    }
}

