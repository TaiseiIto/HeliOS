use bitfield_struct::bitfield;

/// # Redirection Table Entry
/// ## References
/// * [Intel 600 Series and Intel 700 Series Chipset Family Platform Controller Hub (PCH) Datasheet - Volume 2 of 2](https://www.intel.com/content/www/us/en/content-details/680836/intel-600-series-and-intel-700-series-chipset-family-platform-controller-hub-pch-datasheet-volume-2-of-2.html) 24.1.3 Redirection Table Entry 0 (RTE0)
#[bitfield(u64)]
pub struct Entry {
    vector: u8,
    #[bits(3)]
    delivery_mode: u8,
    destination_mode: bool,
    delivery_status: bool,
    polarity: bool,
    remote_irr: bool,
    trigger_mode: bool,
    mask: bool,
    #[bits(31, access = RO)]
    reserved0: u32,
    extended_destination_id: u8,
    destination_id: u8,
}

