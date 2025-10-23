#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}
