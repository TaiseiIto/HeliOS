use bitfield_struct::bitfield;

/// # Link Control and Status Register - 0x090
/// ## References
/// * [Link Control and Status Register - 0x090](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 60. Link Control and Status Register - 0x090
/// * [PCI_EXPRESS_LINK_CONTROL_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_link_control_register)
#[bitfield(u16)]
pub struct Register {
    #[bits(2)]
    aspm_control: u8,
    __: bool,
    read_completion_boundary: bool,
    link_disable: bool,
    retrain_link: bool,
    common_clock_configuration: bool,
    extended_synch: bool,
    enable_clock_power_management: bool,
    #[bits(7)]
    __: u8,
}
