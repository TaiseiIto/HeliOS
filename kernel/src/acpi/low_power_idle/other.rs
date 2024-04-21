#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u32,
    length: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

