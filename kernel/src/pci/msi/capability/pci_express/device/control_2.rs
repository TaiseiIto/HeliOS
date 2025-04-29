use bitfield_struct::bitfield;

/// # PCI Express Device Control 2 and Status 2 Register - 0x0A8
/// ## Referneces
/// * [PCI Express Device Control 2 and Status 2 Register - 0x0A8](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 62. PCI Express Device Control 2 and Status 2 Register - 0x0A8
#[bitfield(u16)]
pub struct Register {
    #[bits(4)]
    completion_timeout_value: u8,
    completion_timeout_disable: bool,
    __: bool,
    atomic_operation_requester_enable: bool,
    #[bits(9)]
    __: u16,
}

