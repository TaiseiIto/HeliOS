use bitfield_struct::bitfield;

/// # Base Address Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.2.4 Expansion ROM Base Address Register (Offset 30h)
#[bitfield(u32)]
pub struct Register {
    enable: bool,
    #[bits(3)]
    validation_status: u8,
    #[bits(4)]
    validation_details: u8,
    #[bits(3)]
    __: u8,
    #[bits(21)]
    base_address: u32,
}

