use bitfield_struct::bitfield;

/// # Processor Priority Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.8.3.1 Figure 11-19. Processor Priority Register (PPR)
#[bitfield(u128)]
pub struct Register {
    #[bits(4)]
    sub_class: u8,
    #[bits(4)]
    class: u8,
    #[bits(120, access = RO)]
    reserved0: u128,
}

