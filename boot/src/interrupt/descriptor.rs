pub mod table;

pub use table::Table;

use {
    bitfield_struct::bitfield,
    crate::x64::descriptor::Type,
};

/// # Interrupt Descriptor
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 6.14.1 64-Bit Mode IDT, Figure 6-8. 64-Bit IDT Gate Descriptors
#[bitfield(u128)]
pub struct Descriptor {
    offset0: u16,
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
    offset1: u64,
    reserved2: u32,
}

#[derive(Debug)]
pub struct Interface {
    #[allow(dead_code)]
    offset: usize,
    #[allow(dead_code)]
    segment_selector: u16,
    #[allow(dead_code)]
    interrupt_stack_table: u8,
    #[allow(dead_code)]
    descriptor_type: Type,
    #[allow(dead_code)]
    dpl: u8,
}

impl From<&Descriptor> for Option<Interface> {
    fn from(descriptor: &Descriptor) -> Self {
        if descriptor.p() {
            let offset0: usize = descriptor.offset0() as usize;
            let offset1: usize = descriptor.offset1() as usize;
            let offset: usize = offset0 + (offset1 << Descriptor::OFFSET0_OFFSET);
            let segment_selector: u16 = descriptor.segment_selector();
            let interrupt_stack_table: u8 = descriptor.ist();
            let descriptor_type: u8 = descriptor.descriptor_type();
            let descriptor_type = Type::new(descriptor_type, false, false, false);
            let dpl: u8 = descriptor.dpl();
            Some(Interface {
                offset,
                segment_selector,
                interrupt_stack_table,
                descriptor_type,
                dpl,
            })
        } else {
            None
        }
    }
}

