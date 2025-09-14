use bitfield_struct::bitfield;

/// # Command Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.1.3 Command Register (Offset 04h)
#[bitfield(u16)]
pub struct Register {
    io_space_enable: bool,
    memory_space_enable: bool,
    bus_master_enable: bool,
    special_cycle_enable: bool,
    memory_write_and_invalidate: bool,
    vga_palette_snoop: bool,
    parity_error_responce: bool,
    idsel_stepping_wait_cycle_control: bool,
    serr_enable: bool,
    fast_back_to_back_transactions_enable: bool,
    interrupt_disable: bool,
    #[bits(5)]
    __: u8,
}
