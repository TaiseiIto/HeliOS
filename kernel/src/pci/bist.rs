use bitfield_struct::bitfield;

/// # BIST Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.1.10 Command Register (Offset 0Fh)
#[bitfield(u8)]
pub struct Register {
    #[bits(4)]
    completion_code: u8,
    #[bits(2)]
    __: u8,
    start_bist: bool,
    bist_capable: bool,
}

