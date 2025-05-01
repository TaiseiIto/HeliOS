use bitfield_struct::bitfield;

/// # Link Capabilities Register - 0x08C
/// ## References
/// * [Link Capabilities Register - 0x08C](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 59. Link Capabilities Register - 0x08C
/// * [PCI_EXPRESS_LINK_CAPABILITIES_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_link_capabilities_register)
#[bitfield(u32)]
pub struct Register {
    #[bits(4)]
    maximum_link_speed: u8,
    #[bits(6)]
    maximum_link_width: u8,
    aspm_support_for_l0s_state: bool,
    aspm_support_for_l1_state: bool,
    #[bits(3)]
    l0s_exit_latency: u8,
    #[bits(3)]
    l1_exit_latency: u8,
    clock_power_management: bool,
    surprise_down_error_reporting_capable: bool,
    data_link_layer_active_reporting_capable: bool,
    link_bandwidth_notification_capability: bool,
    aspm_optionally_compliance: bool,
    __: bool,
    port_number: u8,
}

