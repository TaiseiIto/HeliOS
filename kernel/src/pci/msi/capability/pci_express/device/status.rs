use bitfield_struct::bitfield;

/// # PCI Express Device Control and Status Register - 0x88
/// ## Referneces
/// * [PCI Express Device Control and Status Register - 0x88](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 58. PCI Express Device Control and Status Register - 0x88
#[bitfield(u16)]
pub struct Register {
    correctable_error_detected: bool,
    non_fatal_error_detected: bool,
    fatal_error_detected: bool,
    unsupported_request_detected: bool,
    __: bool,
    transaction_pending: bool,
    #[bits(10)]
    __: u16,
}

