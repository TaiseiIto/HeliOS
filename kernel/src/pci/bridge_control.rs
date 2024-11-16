use bitfield_struct::bitfield;

/// # Bridge Control Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.3.13 Bridge Control Register (Offset 3Eh)
#[bitfield(u16)]
pub struct Register {
    parity_error_responce_enable: bool,
    serr_enable: bool,
    isa_enable: bool,
    vga_enable: bool,
    vga_16bit_decode: bool,
    master_abort_mode: bool,
    secondary_bus_reset: bool,
    fast_back_to_back_transactions_enable: bool,
    primary_discard_timer: bool,
    secondary_discard_timer: bool,
    discard_timer_status: bool,
    discard_timer_serr_enable: bool,
    #[bits(4)]
    __: u8,
}

