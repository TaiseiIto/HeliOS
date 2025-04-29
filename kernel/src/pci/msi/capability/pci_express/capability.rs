use bitfield_struct::bitfield;

/// # PCI Express Capability Register - 0x080
/// ## Referneces
/// * [PCI Express Capability Register - 0x080](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 56. PCI Express Capability Register - 0x080
#[bitfield(u16)]
pub struct Register {
    #[bits(3)]
    version_id: u8,
    #[bits(13)]
    __: u16,
}

