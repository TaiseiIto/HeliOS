use bitfield_struct::bitfield;

/// # TSS Descriptor in 64-Bit mode
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 8.2.3 TSS Descriptor in 64-Bit mode
#[bitfield(u128)]
pub struct Descriptor {
    descriptor: memory::segment::Descriptor,
    base_address: u32,
    #[bits(access = RO)]
    reserved0: u8,
    #[bits(5, access = RO)]
    zero: u8,
    #[bits(19, access = RO)]
    reserved1: u32,
}

