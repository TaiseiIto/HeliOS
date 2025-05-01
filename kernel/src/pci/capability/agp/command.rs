use bitfield_struct::bitfield;

/// # AGP Command Register
/// ## References
/// * [AGP V3.0 Interface Specification](http://www.playtool.com/pages/agpcompat/agp30.pdf) 2.7.5 AGP COMMAND REGISTER
#[bitfield(u32)]
pub struct Register {
    #[bits(3)]
    drate: u8,
    __: bool,
    fw_enable: bool,
    over4g: bool,
    #[bits(2)]
    __: u8,
    agp_enable: bool,
    sba_enable: bool,
    #[bits(3)]
    pcal_cycle: u8,
    #[bits(3)]
    parqsz: u8,
    __: u8,
    prq: u8,
}

