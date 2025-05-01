pub mod byte0;

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
                (Some(small), None) => Some(Self::Small(small)),
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
}

