pub mod table;

pub use table::Table;

use bitfield_struct::bitfield;

/// # Interrupt Descriptor
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.14.1 64-Bit Mode IDT, Figure 6-8. 64-Bit IDT Gate Descriptors
#[bitfield(u128)]
pub struct Descriptor {
    offset_in_segment0: u16,
    segment_selector: u16,
    #[bits(3)]
    ist: u8,
    #[bits(5)]
    reserved0: u8,
    #[bits(4)]
    descriptor_type: u8,
    reserved1: bool,
    #[bits(2)]
    dpl: u8,
    p: bool,
    #[bits(48)]
    offset_in_segment1: u64,
    reserved2: u32,
}

#[derive(Debug)]
pub struct Debug {
}

impl From<&Descriptor> for Option<Debug> {
    fn from(descriptor: &Descriptor) -> Self {
        if descriptor.p() {
            Some(Debug {
            })
        } else {
            None
        }
    }
}

