use {
    alloc::{
        boxed::Box,
        vec::Vec,
    },
    core::{
        fmt,
        ops::Range,
    },
    super::{
        KIB,
        Paging,
    },
};

pub const SIZE: usize = 4 * KIB;

#[derive(Debug)]
pub struct ContinuousPages {
    #[allow(dead_code)]
    pages: Vec<Page>,
    vaddr_range: Range<usize>,
}

impl ContinuousPages {
    pub fn new(paging: &mut Paging, vaddr_range: Range<usize>, writable: bool, executable: bool) -> Self {
        assert!(!vaddr_range.is_empty());
        assert_eq!(vaddr_range.start % SIZE, 0);
        assert_eq!(vaddr_range.end % SIZE, 0);
        let pages: Vec<Page> = vaddr_range
            .clone()
            .step_by(SIZE)
            .map(|vaddr| Page::new(paging, vaddr, writable, executable))
            .collect();
        Self {
            pages,
            vaddr_range,
        }
    }

    pub fn range(&self) -> &Range<usize> {
        &self.vaddr_range
    }
}

#[derive(Debug)]
pub struct Page {
    #[allow(dead_code)]
    page: Box<InHeap>,
    #[allow(dead_code)]
    vaddr: usize,
}

impl Page {
    pub fn new(paging: &mut Paging, vaddr: usize, writable: bool, executable: bool) -> Self {
        let page: Box<InHeap> = Box::default();
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
struct InHeap {
    bytes: [u8; SIZE],
}

impl InHeap {
    fn paddr(&self, paging: &Paging) -> usize {
        paging
            .vaddr2paddr(self)
            .unwrap()
    }

    fn vaddr(&self) -> usize {
        let vaddr: *const u8 = self.bytes
            .as_slice()
            .as_ptr();
        vaddr as usize
    }
}

impl Default for InHeap {
    fn default() -> Self {
        let bytes: [u8; SIZE] = [0; SIZE];
        Self {
            bytes,
        }
    }
}

impl fmt::Debug for InHeap {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("page::InHeap")
            .field("vaddr", &self.vaddr())
            .finish()
    }
}

