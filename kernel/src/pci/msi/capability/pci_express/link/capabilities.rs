use bitfield_struct::bitfield;

/// # Link Capabilities Register - 0x08C
/// ## References
/// * [Link Capabilities Register - 0x08C](https://www.intel.com/content/www/us/en/docs/programmable/683686/20-4/pci-express-capability-structure.html) Table 59. Link Capabilities Register - 0x08C
#[bitfield(u32)]
pub struct Register {
    #[bits(4)]
    maximum_link_speed: u8,
    #[bits(6)]
    maximum_link_width: u8,
    aspm_support_for_l0s_state: bool,
    aspm_support_for_l1_state: bool,
    #[bits(3)]
    l0s_exit_latency: u8,
    #[bits(3)]
    l1_exit_latency: u8,
    #[bits(4)]
    __: u8,
    aspm_optionally_compliance: bool,
    #[bits(9)]
    __: u16,
}

