#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Header {
    #[allow(dead_code)]
    record_type: u16,
    length: u8,
    #[allow(dead_code)]
    revision: u8,
}

impl Header {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

