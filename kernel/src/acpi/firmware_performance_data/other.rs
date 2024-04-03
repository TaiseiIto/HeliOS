use super::record;

#[derive(Debug)]
#[repr(packed)]
pub struct Record {
    header: record::Header,
}

impl Record {
    pub fn length(&self) -> usize {
        self.header.length()
    }
}

