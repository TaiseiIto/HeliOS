use bitfield_struct::bitfield;

/// # Divide Configuration Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.5.4 Figure 11-10. Divide Configuration Register
#[bitfield(u128)]
pub struct Register {
    #[bits(2)]
    divide_value0: u8,
    #[bits(access = RO)]
    reserved0: bool,
    divide_value1: bool,
    #[bits(124, access = RO)]
    reserved1: u128,
}

