use bitfield_struct::bitfield;

/// # Local APIC Version Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.4.8 Local APIC Version Register
#[bitfield(u128)]
pub struct Register {
    version: u8,
    #[bits(access = RO)]
    reserved0: u8,
    max_lvt_entry: u8,
    support_for_eoi_broadcast_suppression: bool,
    #[bits(103, access = RO)]
    reserved1: u128,
}

