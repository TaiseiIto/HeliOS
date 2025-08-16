use bitfield_struct::bitfield;

/// # PMC - Power Management Capabilities (Offset = 2)
/// ## References
/// * [PCI Power Management Interface Specification Revision 1.2](https://lekensteyn.nl/files/docs/PCI_Power_Management_12.pdf) 3.2.3. PMC - Power Management Capabilities (Offset = 2)
#[bitfield(u16)]
pub struct Register {
    #[bits(3)]
    version: u8,
    pme_clock: bool,
    __: bool,
    dsi: bool,
    #[bits(3)]
    aux_current: u8,
    d1_support: bool,
    d2_support: bool,
    #[bits(5)]
    pme_support: u8,
}

