use bitfield_struct::bitfield;

/// # Error Status Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.3 Figure 11-9. Error Status Register (ESR)
#[bitfield(u128)]
pub struct Register {
    send_checksum_error: bool,
    receive_checksum_error: bool,
    send_accept_error: bool,
    receive_accept_error: bool,
    redirectable_ipi: bool,
    send_illegal_vector: bool,
    received_illegal_vector: bool,
    illegal_register_address: bool,
    #[bits(120, access = RO)]
    reserved0: u128,
}

