#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Header {
    record_type: u16,
    length: u8,
    revision: u8,
}

impl Header {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

