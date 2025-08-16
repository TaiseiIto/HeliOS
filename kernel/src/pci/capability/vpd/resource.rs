use {
    alloc::{
        string::String,
        vec::Vec,
    },
    core::fmt,
};

pub mod byte0;
pub mod item;

pub struct Data {
    header: Header,
    data: Vec<u8>,
}

impl Data {
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
        let header: &Header = &self.header;
        let data: Type = self.into();
        formatter
            .debug_struct("Data")
            .field("header", header)
            .field("data", &data)
            .finish()
    }
}

#[derive(Debug)]
pub enum Type {
    String(String),
    Items(Vec<item::Format>),
    End,
}

impl From<&Data> for Type {
    fn from(data: &Data) -> Self {
        let Data {
            header,
            data,
        } = data;
        match header.tag() {
            Tag::IdentifierString => Self::String(String::from_utf8(data.to_vec()).unwrap()),
            Tag::VpdR | Tag::VpdW => Self::Items(item::FormatIterator::new(data
                    .iter()
                    .cloned())
                .collect()),
            Tag::End => Self::End,
        }
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

