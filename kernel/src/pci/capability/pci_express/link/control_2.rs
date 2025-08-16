use bitfield_struct::bitfield;

/// # Link Control 2 and Status 2 Register - 0x0B0
/// ## References
/// * [Link Control 2 and Status 2 Register - 0x0B0](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 64. Link Control 2 and Status 2 Register - 0x0B0
/// * [PCI_EXPRESS_LINK_CONTROL_2_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-pci_express_link_control_2_register)
#[bitfield(u16)]
pub struct Register {
    #[bits(4)]
    target_link_speed: u8,
    enter_compliance: bool,
    hardware_autonomous_speed_disable: bool,
    selectable_deemphasis: bool,
    #[bits(3)]
    transmit_margin: u8,
    enter_modified_compliance: bool,
    compliance_sos: bool,
    #[bits(4)]
    compliance_preset_deemphasis: u8,
}

