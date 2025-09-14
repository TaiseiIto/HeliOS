use bitfield_struct::bitfield;

/// # PCI Express Device Capabilities Register - 0x084
/// ## Referneces
/// * [PCI Express Device Capabilities Register - 0x084](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 57. PCI Express Device Capabilities Register - 0x084
/// * [PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_device_capabilities_register)
#[bitfield(u32)]
pub struct Register {
    #[bits(3)]
    maximum_payload_size_supported_by_the_function: u8,
    #[bits(2)]
    phantom_functions_supported: u8,
    extended_tag_supported: bool,
    #[bits(3)]
    acceptable_l0s_latency: u8,
    #[bits(3)]
    acceptable_l1_latency: u8,
    #[bits(3)]
    __: u8,
    role_based_error_reporting_supported: bool,
    #[bits(2)]
    __: u8,
    captured_slot_power_limit: u8,
    #[bits(2)]
    captured_slot_power_limit_scale: u8,
    flr_capable: bool,
    #[bits(3)]
    __: u8,
}
