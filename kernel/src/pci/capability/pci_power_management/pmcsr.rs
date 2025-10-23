use bitfield_struct::bitfield;

/// # PMCSR - Power Management Control/Status (Offset = 4)
/// ## References
/// * [PCI Power Management Interface Specification Revision 1.2](https://lekensteyn.nl/files/docs/PCI_Power_Management_12.pdf) 3.2.4. PMCSR - Power Management Control/Status (Offset = 4)
#[bitfield(u16)]
pub struct Register {
    #[bits(2)]
    power_state: u8,
    __: bool,
    no_soft_reset: bool,
    #[bits(4)]
    __: u8,
    pme_en: bool,
    #[bits(4)]
    data_select: u8,
    #[bits(2)]
    data_scale: u8,
    pme_status: bool,
}
