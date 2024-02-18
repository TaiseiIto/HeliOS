use {
    alloc::boxed::Box,
    super::{
        KIB,
        Paging,
    },
};

pub const SIZE: usize = 4 * KIB;

pub struct Interface {
    page: Box<Page>,
    vaddr: usize,
}

impl Interface {
    pub fn new(paging: &mut Paging, vaddr: usize, writable: bool, executable: bool) -> Self {
        let page: Box<Page> = Box::default();
        let paddr: usize = page
            .as_ref()
            .paddr(paging);
        let present: bool = true;
        paging.set_page(vaddr, paddr, present, writable, executable);
        Self {
            page,
            vaddr,
        }
    }
}

#[repr(align(4096))]
struct Page {
    bytes: [u8; SIZE],
}

impl Page {
    fn paddr(&self, paging: &Paging) -> usize {
        paging
            .vaddr2paddr(self)
            .unwrap()
    }
}

impl Default for Page {
    fn default() -> Self {
        let bytes: [u8; SIZE] = [0; SIZE];
        Self {
            bytes,
        }
    }
}

