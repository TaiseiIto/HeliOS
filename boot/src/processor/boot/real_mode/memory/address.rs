use {
    super::super::{segment, Register},
    core::ops::Range,
};

pub const BITS: usize = segment::SHIFT + (Register::BITS as usize);
pub const END: usize = 1 << BITS;
pub const RANGE: Range<usize> = 0..END;
