use {
    core::ops::Range,
    super::{
        ContinuousPages,
        Paging,
        page,
    },
};

#[derive(Debug)]
pub struct Stack {
    #[allow(dead_code)]
    pages: ContinuousPages,
}

impl Stack {
    pub fn floor(&self) -> usize {
        self.pages.range().end
    }

    pub fn new(paging: &mut Paging, floor: usize, pages: usize) -> Self {
        let size: usize = pages * page::SIZE;
        let ceil: usize = floor - size;
        let range: Range<usize> = ceil..floor;
        let writable: bool = true;
        let executable: bool = false;
        let pages = ContinuousPages::new(paging, range, writable, executable);
        Self {
            pages,
        }
    }
}

