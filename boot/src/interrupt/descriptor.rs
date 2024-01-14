pub mod table;

pub use table::Table;

use bitfield_struct::bitfield;

/// # Interrupt Descriptor
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 5.8.3.1 IA-32e Mode Call Gates, Figure 5-9. Call-Gate Descriptor in IA-32e Mode
#[bitfield(u128)]
pub struct Descriptor {
    offset_in_segment0: u16,
    segment_selector: u16,
    reserved0: u8,
    #[bits(4)]
    type0: u8,
    reserved1: bool,
    #[bits(2)]
    dpl: u8,
    p: bool,
    #[bits(48)]
    offset_in_segment1: u64,
    reserved2: u8,
    #[bits(5)]
    type1: u8,
    #[bits(19)]
    reserved: u32,
}

