use bitfield_struct::bitfield;

/// # PCI Express Device Capabilities 2 Register - 0x0A4
/// ## Referneces
/// * [PCI Express Device Capabilities 2 Register - 0x0A4](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 61. PCI Express Device Capabilities 2 Register - 0x0A4
#[bitfield(u32)]
pub struct Register {
    #[bits(4)]
    completion_timeout_ranges: u8,
    completion_timeout_disable_supported: bool,
    #[bits(27)]
    __: u32,
}

