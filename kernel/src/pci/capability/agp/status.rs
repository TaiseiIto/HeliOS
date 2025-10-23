use bitfield_struct::bitfield;

/// # AGP Status Register
/// ## References
/// * [AGP V3.0 Interface Specification](http://www.playtool.com/pages/agpcompat/agp30.pdf) 2.7.4 AGP STATUS REGISTER
#[bitfield(u32)]
pub struct Register {
    #[bits(3)]
    rate: u8,
    agp30_mode: bool,
    fw: bool,
    over4g: bool,
    htrans: bool,
    #[bits(3)]
    __: u8,
    #[bits(3)]
    cal_cycle: u8,
    #[bits(3)]
    arqsz: u8,
    __: u8,
    rq: u8,
}
