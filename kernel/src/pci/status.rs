use bitfield_struct::bitfield;

/// # Status Register
/// ## References
/// * [PCI Express Base Specification Revision 5.0 Version 1.0](https://picture.iczhiku.com/resource/eetop/SYkDTqhOLhpUTnMx.pdf) 7.5.1.1.4 Status Register (Offset 06h)
#[bitfield(u16)]
pub struct Register {
    immediate_readiness: bool,
    #[bits(2, access = RO)]
    reserved0: u8,
    interrupt_status: bool,
    capabilities_list: bool,
    capable_66mhz: bool,
    #[bits(access = RO)]
    reserved1: bool,
    fast_back_to_back_transactions_capable: bool,
    master_data_parity_error: bool,
    #[bits(2)]
    devsel_timing: u8,
    signaled_target_abort: bool,
    received_target_abort: bool,
    received_master_abort: bool,
    signaled_system_error: bool,
    detected_parity_error: bool,
}

