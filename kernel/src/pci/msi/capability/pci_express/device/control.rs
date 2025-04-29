use bitfield_struct::bitfield;

/// # PCI Express Device Control and Status Register - 0x088
/// ## Referneces
/// * [PCI Express Device Control and Status Register - 0x088](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 58. PCI Express Device Control and Status Register - 0x088
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
    #[bits(2)]
    __: u8,
    enable_no_snoop: bool,
    #[bits(3)]
    maximum_read_request_size: u8,
    function_level_reset: bool,
}

