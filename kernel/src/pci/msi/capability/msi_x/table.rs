use bitfield_struct::bitfield;

/// # Table Offset/Table BIR for MSI-X
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2.4. Table Offset/Table BIR for MSI-X
#[bitfield(u32)]
pub struct Register {
    #[bits(3)]
    bir: u8,
    #[bits(29)]
    offset: u32,
}

/// # Table Entry
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2. Fugure 6-11: MSI-X Table Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Entry {
    msg_addr: u64,
    msg_data: u32,
    vector_control: VectorControl,
}

/// # Vector Control
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2.9. Vector Control for MSI-X Table Entries
#[bitfield(u32)]
pub struct VectorControl {
    mask_bit: bool,
    #[bits(31)]
    __: u32,
}

