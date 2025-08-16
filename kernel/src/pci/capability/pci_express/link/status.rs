use bitfield_struct::bitfield;

/// # Link Control and Status Register - 0x090
/// ## References
/// * [Link Control and Status Register - 0x090](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 60. Link Control and Status Register - 0x090
/// * [PCI_EXPRESS_LINK_STATUS_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_link_status_register)
#[bitfield(u16)]
pub struct Register {
    #[bits(4)]
    negotiated_link_speed: u8,
    #[bits(6)]
    negotiated_link_width: u8,
    __: bool,
    link_training: bool,
    slot_clock_configuration: bool,
    data_link_layer_active: bool,
    #[bits(2)]
    __: u8,
}

