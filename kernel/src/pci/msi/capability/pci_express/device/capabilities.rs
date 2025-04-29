use bitfield_struct::bitfield;

/// # PCI Express Device Capabilities Register - 0x84
/// ## Referneces
/// * [PCI Express Device Capabilities Register - 0x84](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 57. PCI Express Device Capabilities Register - 0x84
#[bitfield(u32)]
pub struct Register {
    #[bits(3)]
    maximum_payload_size_supported_by_the_function: u8,
    #[bits(2)]
    __: u8,
    extended_tag_supported: bool,
    #[bits(3)]
    acceptable_los_latency: u8,
    #[bits(3)]
    acceptable_l1_latency: u8,
    #[bits(3)]
    __: u8,
    role_based_error_reporting_supported: bool,
    #[bits(2)]
    __: u8,
    #[bits(10)]
    captured_slot_power_limit_value_and_scale: u16,
    flr_capable: bool,
    #[bits(3)]
    __: u8,
}

