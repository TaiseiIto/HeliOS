pub mod message;

use {
    core::fmt,
    super::Header,
};

/// # MSI Capability Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.1. MSI Capability Structures
#[repr(packed)]
pub struct Structure {
    header: Header,
    message_control: message::Control,
    message_address: u64,
    message_data: u16,
    __: u16,
    mask_bits: u32,
    pending_bits: u32,
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let capability_id: u8 = header.capability_id();
        let next_pointer: u8 = header.next_pointer();
        let message_control: message::Control = self.message_control;
        let message_address: u64 = self.message_address;
        let message_data: u16 = self.message_data;
        let mask_bits: u32 = self.mask_bits;
        let pending_bits: u32 = self.pending_bits;
        formatter
            .debug_struct("Structure")
            .field("capability_id", &capability_id)
            .field("next_pointer", &next_pointer)
            .field("message_control", &message_control)
            .field("message_address", &message_address)
            .field("message_data", &message_data)
            .field("mask_bits", &mask_bits)
            .field("pending_bits", &pending_bits)
            .finish()
    }
}

