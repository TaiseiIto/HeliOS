use bitfield_struct::bitfield;

/// # Version Register
/// ## References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.1.2 Version Register (VER)
#[bitfield(u32)]
pub struct Register {
    version: u8,
    #[bits(7, access = RO)]
    reserved0: u8,
    pin_assertion_register_supported: bool,
    maximum_redirection_entries: u8,
    reserved1: u8,
}

