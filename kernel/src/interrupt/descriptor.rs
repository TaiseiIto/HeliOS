pub mod table;

pub use table::Table;

use {
    bitfield_struct::bitfield,
    crate::{
        memory,
        x64,
    },
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
    #[bits(5, access = RO)]
    reserved0: u8,
    #[bits(4)]
    descriptor_type: u8,
    #[bits(access = RO)]
    reserved1: bool,
    #[bits(2)]
    dpl: u8,
    p: bool,
    #[bits(48)]
    offset1: u64,
    #[bits(access = RO)]
    reserved2: u32,
}

impl From<&Interface> for Descriptor {
    fn from(interface: &Interface) -> Self {
        let Interface {
            offset,
            segment_selector,
            interrupt_stack_table,
            descriptor_type,
            dpl,
        } = interface;
        let offset0: u16 = (offset & ((1 << Self::OFFSET0_BITS) - 1)) as u16;
        let offset1: u64 = (offset >> Self::OFFSET0_BITS) as u64;
        let segment_selector: u16 = (*segment_selector).into();
        let ist: u8 = *interrupt_stack_table;
        let descriptor_type: u8 = descriptor_type.segment_type();
        let dpl: u8 = *dpl;
        Self::default()
            .with_offset0(offset0)
            .with_segment_selector(segment_selector)
            .with_ist(ist)
            .with_descriptor_type(descriptor_type)
            .with_dpl(dpl)
            .with_p(true)
            .with_offset1(offset1)
    }
}

#[derive(Debug)]
pub struct Interface {
    offset: usize,
    segment_selector: memory::segment::Selector,
    interrupt_stack_table: u8,
    descriptor_type: x64::descriptor::Type,
    dpl: u8,
}

impl Interface {
    pub fn new(handler: extern "x86-interrupt" fn(), interrupt_stack_table: u8) -> Self {
        let offset: usize = handler as usize;
        let segment_selector = memory::segment::Selector::cs();
        let descriptor_type = x64::descriptor::Type::interrupt_gate();
        let dpl: u8 = segment_selector.get_rpl();
        Self {
            offset,
            segment_selector,
            interrupt_stack_table,
            descriptor_type,
            dpl,
        }
    }
}

impl From<&Descriptor> for Option<Interface> {
    fn from(descriptor: &Descriptor) -> Self {
        descriptor.p().then(|| {
            let offset0: usize = descriptor.offset0() as usize;
            let offset1: usize = descriptor.offset1() as usize;
            let offset: usize = offset0 + (offset1 << Descriptor::OFFSET0_BITS);
            let segment_selector: memory::segment::Selector = descriptor
                .segment_selector()
                .into();
            let interrupt_stack_table: u8 = descriptor.ist();
            let descriptor_type: u8 = descriptor.descriptor_type();
            let descriptor_type = x64::descriptor::Type::new(descriptor_type, false, false, false);
            let dpl: u8 = descriptor.dpl();
            Interface {
                offset,
                segment_selector,
                interrupt_stack_table,
                descriptor_type,
                dpl,
            }
        })
    }
}

