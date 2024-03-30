#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u16,
    length: u16,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

