use {
    alloc::{
        string::String,
        vec,
        vec::Vec,
    },
    super::Data,
};

/// # VPD Format
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I.1. VPD Format. Figure I-5: VPD Format
pub struct Format {
    keyword: String,
    length: u8,
    data: Vec<u8>,
}

impl Format {
    fn from_byte_iterator<T>(byte_iterator: &mut T) -> Option<Self> where T: Iterator<Item = u8> {
        let keyword_low: Option<u8> = byte_iterator.next();
        let keyword_high: Option<u8> = byte_iterator.next();
        let length: Option<u8> = byte_iterator.next();
        match (keyword_low, keyword_high, length) {
            (Some(keyword_low), Some(keyword_high), Some(length)) => {
                let keyword: Vec<u8> = vec![keyword_low, keyword_high];
                let keyword: String = String::from_utf8(keyword).unwrap();
                let data: Vec<u8> = (0..length)
                    .map(|_| byte_iterator
                        .next()
                        .unwrap())
                    .collect();
                Some(Self {
                    keyword,
                    length,
                    data,
                })
            },
            _ => None,
        }
    }
}

pub struct FormatIterator<T> where T: Iterator<Item = u8> {
    byte_iterator: T,
}

impl<T> Iterator for FormatIterator<T> where T: Iterator<Item = u8> {
    type Item = Format;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            byte_iterator,
        } = self;
        Self::Item::from_byte_iterator(byte_iterator)
    }
}

