use bitfield_struct::bitfield;

/// # Link Control 2 and Status 2 Register - 0x0B0
/// ## References
/// * [Link Control 2 and Status 2 Register - 0x0B0](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 64. Link Control 2 and Status 2 Register - 0x0B0
/// * [PCI_EXPRESS_LINK_STATUS_2_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-pci_express_link_status_2_register)
#[bitfield(u16)]
pub struct Register {
    current_deemphasis_level: bool,
    equalization_complete: bool,
    equalization_phase_1_successfull: bool,
    equalization_phase_2_successfull: bool,
    equalization_phase_3_successfull: bool,
    link_equalization_request: bool,
    #[bits(10)]
    __: u16,
}

