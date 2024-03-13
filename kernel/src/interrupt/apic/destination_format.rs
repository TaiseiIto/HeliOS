use bitfield_struct::bitfield;

/// # Destinatio Format Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.6.2 Figure 11-14. Destination Format Register (DFR)
#[bitfield(u128)]
pub struct Register {
    #[bits(28, access = RO)]
    reserved0: u32,
    #[bits(4)]
    model: u8,
    #[bits(96, access = RO)]
    reserved1: u128,
}

