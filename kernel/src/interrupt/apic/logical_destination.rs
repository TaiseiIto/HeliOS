use bitfield_struct::bitfield;

/// # Logical Destination Register
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) 3.11.6.2.2 Figure 11-13. Logical Destination Register (LDR)
#[bitfield(u128)]
pub struct Register {
    #[bits(24, access = RO)]
    reserved0: u32,
    logical_apic_id: u8,
    #[bits(96)]
    reserved1: u128,
}

