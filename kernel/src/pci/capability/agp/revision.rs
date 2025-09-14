use bitfield_struct::bitfield;

/// # AGP Revision Register
/// ## References
/// * [AGP V3.0 Interface Specification](http://www.playtool.com/pages/agpcompat/agp30.pdf) 2.7.3 AGP IDENTIFIER REGISTER
#[bitfield(u16)]
pub struct Register {
    #[bits(4)]
    minor: u8,
    #[bits(4)]
    major: u8,
    __: u8,
}
