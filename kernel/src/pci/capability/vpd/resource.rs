pub mod byte0;
pub mod item;

use {
    alloc::vec::Vec,
    core::{
        fmt,
        str,
    },
};

pub struct Data {
    header: Header,
    data: Vec<u8>,
}

impl Data {
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn from_byte_iterator<T>(byte_iterator: &mut T) -> Option<Self> where T: Iterator<Item = u8> {
        Header::from_byte_iterator(byte_iterator).map(|header| {
            let length: u16 = header.length();
            let data: Vec<u8> = (0..length)
                .map(|_| byte_iterator
                    .next()
                    .unwrap())
                .collect();
            Self {
                header,
                data,
            }
        })
    }

    pub fn header(&self) -> &Header {
        &self.header
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            header,
            data,
        } = self;
        let mut debug_struct: fmt::DebugStruct = formatter.debug_struct("Data");
        debug_struct.field("header", header);
        match header.tag() {
            Tag::IdentifierString => {
                debug_struct.field("data", &str::from_utf8(data).unwrap());
            },
            Tag::VpdR => {},
            Tag::VpdW => {},
            Tag::End => {},
        }
        debug_struct.finish()
    }
}

/// # Small Resource Data Type Tag Bit Definitions
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I. Vital Product Data. Figure I-2: Small Resource Data Type Tag Bit Definitions
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I. Vital Product Data. Figure I-3: Large Resource Data Type Tag Bit Definitions
pub enum Header {
    Small(byte0::Small),
    Large {
        tag: byte0::Large,
        length: u16,
    },
}

impl Header {
    pub fn from_byte_iterator<T>(byte_iterator: &mut T) -> Option<Self> where T: Iterator<Item = u8> {
        byte_iterator
            .next()
            .and_then(|byte0| match (byte0::Small::try_from(byte0), byte0::Large::try_from(byte0)) {
                (Some(byte0), None) => Some(Self::Small(byte0)),
                (None, Some(tag)) => {
                    let length_low: Option<u8> = byte_iterator.next();
                    let length_high: Option<u8> = byte_iterator.next();
                    length_low
                        .zip(length_high)
                        .map(|(length_low, length_high)| {
                            let length_low: u16 = length_low as u16;
                            let length_high: u16 = length_high as u16;
                            let length: u16 = length_low | (length_high << u8::BITS);
                            Self::Large {
                                tag,
                                length,
                            }
                        })
                },
                _ => unreachable!(),
            })
    }

    pub fn length(&self) -> u16 {
        match self {
            Self::Small(byte0) => byte0.get_length() as u16,
            Self::Large {
                tag: _,
                length,
            } => *length,
        }
    }

    pub fn tag(&self) -> Tag {
        match self {
            Self::Small(byte0) => byte0.get_tag(),
            Self::Large {
                tag,
                length: _,
            } => tag.get_tag(),
        }.into()
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Header")
            .field("tag", &self.tag())
            .field("length", &self.length())
            .finish()
    }
}

/// # Resource Data Type Flags for a Typical VPD
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I. Vital Product Data. Figure I-4: Resource Data Type Flags for a Typical VPD
#[derive(Eq, Debug, PartialEq)]
pub enum Tag {
    IdentifierString,
    VpdR,
    VpdW,
    End,
}

impl From<u8> for Tag {
    fn from(tag: u8) -> Self {
        match tag {
            0x02 => Self::IdentifierString,
            0x10 => Self::VpdR,
            0x11 => Self::VpdW,
            0x0f => Self::End,
            tag => unreachable!("tag = {:#x?}", tag),
        }
    }
}

