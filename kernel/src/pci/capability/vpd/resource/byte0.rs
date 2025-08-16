use bitfield_struct::bitfield;

/// # Small Resource Data Type Tag Bit Definitions
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I. Vital Product Data. Figure I-2: Small Resource Data Type Tag Bit Definitions
#[bitfield(u8)]
pub struct Small {
    #[bits(3)]
    length: u8,
    #[bits(4)]
    tag: u8,
    is_large: bool,
}

impl Small {
    pub fn try_from(byte0: u8) -> Option<Self> {
        let byte0: Self = byte0.into();
        (!byte0.is_large()).then_some(byte0)
    }

    pub fn get_length(&self) -> u8 {
        self.length()
    }

    pub fn get_tag(&self) -> u8 {
        self.tag()
    }
}

/// # Large Resource Data Type Tag Bit Definitions
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I. Vital Product Data. Figure I-3: Large Resource Data Type Tag Bit Definitions
#[bitfield(u8)]
pub struct Large {
    #[bits(7)]
    tag: u8,
    is_large: bool,
}

impl Large {
    pub fn try_from(byte0: u8) -> Option<Self> {
        let byte0: Self = byte0.into();
        byte0.is_large().then_some(byte0)
    }

    pub fn get_tag(&self) -> u8 {
        self.tag()
    }
}

