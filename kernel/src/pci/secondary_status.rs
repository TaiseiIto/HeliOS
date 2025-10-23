use bitfield_struct::bitfield;

/// # Secondary Status Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.1.7 Secondary Status Register (Offset 1Eh)
#[bitfield(u16)]
pub struct Register {
    #[bits(5)]
    __: u8,
    capable_66mhz: bool,
    __: bool,
    fast_back_to_back_transactions_capable: bool,
    master_data_parity_error: bool,
    #[bits(2)]
    devsel_timing: u8,
    signaled_target_abort: bool,
    received_target_abort: bool,
    received_master_abort: bool,
    received_system_error: bool,
    detected_parity_error: bool,
}
