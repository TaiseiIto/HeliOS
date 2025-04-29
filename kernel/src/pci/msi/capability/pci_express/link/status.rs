use bitfield_struct::bitfield;

/// # Link Control and Status Register - 0x090
/// ## References
/// * [Link Control and Status Register - 0x090](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 60. Link Control and Status Register - 0x090
#[bitfield(u16)]
pub struct Register {
    #[bits(4)]
    negotiated_link_speed: u8,
    #[bits(6)]
    negotiated_link_width: u8,
    #[bits(2)]
    __: u8,
    slot_clock_configuration: bool,
    #[bits(3)]
    __: u8,
}

