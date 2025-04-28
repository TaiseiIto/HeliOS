use bitfield_struct::bitfield;

/// # MSI Capability Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.1.3. Message Control for MSI
#[bitfield(u16)]
pub struct Control {
    msi_enable: bool,
    #[bits(3)]
    multiple_message_capable: u8,
    #[bits(3)]
    multiple_message_enable: u8,
    bit64_address_capable: bool,
    per_vector_masking_capable: bool,
    #[bits(7)]
    __: u8,
}

