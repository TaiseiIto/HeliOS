use bitfield_struct::bitfield;

/// # Header Type Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.1.9 Command Register (Offset 0Eh)
#[bitfield(u8)]
pub struct Register {
    #[bits(7)]
    header_layout: u8,
    multi_function_device: bool,
}
