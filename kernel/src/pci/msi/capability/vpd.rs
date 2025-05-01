use {
    core::fmt,
    super::Header,
};

/// # VPD Capability Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I. Vital Product Data. Figure I-1. VPD Capability Structure
#[repr(packed)]
pub struct Structure {
    header: Header,
    address: u16,
    data: u32,
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let address: u16 = self.address;
        let data: u32 = self.data;
        formatter
            .debug_struct("Structure")
            .field("header", &header)
            .field("address", &address)
            .field("data", &data)
            .finish()
    }
}

