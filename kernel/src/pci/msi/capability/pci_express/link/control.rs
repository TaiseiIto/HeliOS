use bitfield_struct::bitfield;

/// # Link Control and Status Register - 0x090
/// ## References
/// * [Link Control and Status Register - 0x090](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 60. Link Control and Status Register - 0x090
#[bitfield(u16)]
pub struct Register {
    #[bits(2)]
    aspm_control: u8,
    __: bool,
    read_completion_boundary: bool,
    #[bits(2)]
    __: u8,
    common_clock_configuration: bool,
    extended_synch: bool,
    __: u8,
}

