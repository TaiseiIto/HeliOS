use alloc::vec::Vec;

#[derive(Debug)]
pub struct Loader {
    binary: Vec<u8>,
    base: usize,
    stack_floor: usize,
}

impl Loader {
    pub fn new(binary: Vec<u8>, base: usize, stack_floor: usize) -> Self {
        Self {
            binary,
            base,
            stack_floor,
        }
    }
}

