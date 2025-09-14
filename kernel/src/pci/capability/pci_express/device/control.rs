use bitfield_struct::bitfield;

/// # PCI Express Device Control and Status Register - 0x088
/// ## Referneces
/// * [PCI Express Device Control and Status Register - 0x088](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 58. PCI Express Device Control and Status Register - 0x088
/// * [PCI_EXPRESS_DEVICE_CONTROL_REGISTER union (ntddk.h)](https://learn.microsoft.com/en-us/windows-hardware/drivers/ddi/ntddk/ns-ntddk-_pci_express_device_control_register)
#[bitfield(u16)]
pub struct Register {
    enable_correctable_error_reporting: bool,
    enable_non_fatal_error_reporting: bool,
    enable_fatal_error_reporting: bool,
    enable_unsupported_request_reporting: bool,
    enable_relaxed_ordering: bool,
    #[bits(3)]
    maximum_payload_size: u8,
    extended_tag_field_enable: bool,
    phantom_functions_enable: bool,
    aux_power_enable: bool,
    enable_no_snoop: bool,
    #[bits(3)]
    maximum_read_request_size: u8,
    function_level_reset_or_bridge_config_retry_enable: bool,
}
