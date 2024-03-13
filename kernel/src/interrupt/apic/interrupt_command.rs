use bitfield_struct::bitfield;

/// # Interrupt Command Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.1 Figure 11-12. Interrupt Command Register (ICR)
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Register {
    low: Low,
    high: High,
}

#[bitfield(u128)]
struct Low {
    vector: u8,
    #[bits(3)]
    delivery_mode: u8,
    destination_mode: bool,
    delivery_status: bool,
    #[bits(access = RO)]
    reserved0: bool,
    level: bool,
    trigger_mode: bool,
    #[bits(2, access = RO)]
    reserved1: u8,
    #[bits(2)]
    destination_shorthand: u8,
    #[bits(108, access = RO)]
    reserved2: u128
}

#[bitfield(u128)]
struct High {
    #[bits(24, access = RO)]
    reserved0: u32,
    destination_field: u8,
    #[bits(96)]
    reserved1: u128,
}

