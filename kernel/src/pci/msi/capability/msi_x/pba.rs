use bitfield_struct::bitfield;

/// # PBA Offset/PBA BIR for MSI-X
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2.4. PBA Offset/PBA BIR for MSI-X
#[bitfield(u32)]
pub struct Register {
    #[bits(3)]
    bir: u8,
    #[bits(29)]
    offset: u32,
}

/// # Pending Bits
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2.10. Pending Bits for MSI-X PBA-Entries
pub struct PendingBits(u64);

