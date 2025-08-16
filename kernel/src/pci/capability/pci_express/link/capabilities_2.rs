use bitfield_struct::bitfield;

/// # Link Capabilities 2 Register - 0x0AC
/// ## References
/// * [Link Capabilities 2 Register - 0x0AC](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 63. Link Capabilities 2 Register - 0x0AC
/// * [PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-pci_express_link_capabilities_2_register)
#[bitfield(u32)]
pub struct Register {
    __: bool,
    #[bits(7)]
    link_speeds_supported: u8,
    #[bits(24)]
    __: u32,
}

