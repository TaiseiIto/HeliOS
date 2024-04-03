#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

