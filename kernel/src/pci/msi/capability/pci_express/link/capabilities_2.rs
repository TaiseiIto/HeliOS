use bitfield_struct::bitfield;

/// # Link Capabilities 2 Register - 0x0AC
/// ## References
/// * [Link Capabilities 2 Register - 0x0AC](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 63. Link Capabilities 2 Register - 0x0AC
#[bitfield(u32)]
pub struct Register {
    __: bool,
    #[bits(3)]
    link_speeds_supported: u8,
    #[bits(28)]
    __: u32,
}

