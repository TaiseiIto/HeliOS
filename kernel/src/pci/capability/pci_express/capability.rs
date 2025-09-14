use bitfield_struct::bitfield;

/// # PCI Express Capability Register - 0x080
/// ## Referneces
/// * [PCI Express Capability Register - 0x080](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 56. PCI Express Capability Register - 0x080
/// * [PCI_EXPRESS_CAPABILITY_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_capabilities_register)
#[bitfield(u16)]
pub struct Register {
    #[bits(4)]
    capability_version: u8,
    #[bits(4)]
    device_type: u8,
    slot_implemented: bool,
    #[bits(5)]
    interrupt_message_number: u8,
    #[bits(2)]
    __: u8,
}
