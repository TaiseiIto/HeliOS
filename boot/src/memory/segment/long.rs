use {
    bitfield_struct::bitfield,
    crate::memory,
    super::{
        descriptor,
        short,
    },
};

/// # TSS and LDT Descriptor in 64-Bit mode
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 8.2.3 Figure 8-4. Format of TSS and LDT Descriptors in 64-bit Mode
#[bitfield(u128)]
pub struct Descriptor {
    descriptor: u64,
    base: u32,
    #[bits(access = RO)]
    reserved0: u8,
    #[bits(5, access = RO)]
    zero: u8,
    #[bits(19, access = RO)]
    reserved1: u32,
}

impl Descriptor {
    pub fn base_address(&self) -> Option<usize> {
        let higher_base_address: usize = (self.base() as usize) << u32::BITS;
        let lower_descriptor: short::Descriptor = self.lower_descriptor();
        let lower_descriptor: Option<descriptor::Interface> = (&lower_descriptor).into();
        lower_descriptor.map(|lower_descriptor| lower_descriptor.base() + higher_base_address)
    }

    pub fn lower_descriptor(&self) -> short::Descriptor {
        self.descriptor().into()
    }
}

