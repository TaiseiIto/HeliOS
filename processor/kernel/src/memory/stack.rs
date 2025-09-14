use {
    super::{page, ContinuousPages, Paging},
    core::ops::RangeInclusive,
};

#[derive(Debug)]
pub struct Stack {
    pages: ContinuousPages,
}

impl Stack {
    pub fn new(paging: &mut Paging, floor_inclusive: usize, pages: usize) -> Self {
        let size: usize = pages * page::SIZE;
        let ceil: usize = floor_inclusive - size + 1;
        let range: RangeInclusive<usize> = ceil..=floor_inclusive;
        let writable: bool = true;
        let executable: bool = false;
        let pages = ContinuousPages::new(paging, range, writable, executable);
        Self { pages }
    }

    pub fn wrapping_floor(&self) -> usize {
        self.pages.range_inclusive().end().wrapping_add(1)
    }
}
