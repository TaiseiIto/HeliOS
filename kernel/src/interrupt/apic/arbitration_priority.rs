use bitfield_struct::bitfield;

/// # Arbitration Priority Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.2.4 Figure 11-15. Arbitration Priority Register (APR)
#[bitfield(u128)]
pub struct Register {
    #[bits(4)]
    sub_class: u8,
    #[bits(4)]
    class: u8,
    #[bits(120, access = RO)]
    reserved0: u128,
}

