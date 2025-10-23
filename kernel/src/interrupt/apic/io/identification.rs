use bitfield_struct::bitfield;

/// # Identification Register
/// ## References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.1.1 Identification Register (ID)
#[bitfield(u32)]
pub struct Register {
    #[bits(15)]
    __: u16,
    scratchpad: bool,
    __: u8,
    #[bits(4)]
    apic_identification: u8,
    #[bits(4)]
    __: u8,
}
