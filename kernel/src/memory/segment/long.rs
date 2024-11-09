use {
    bitfield_struct::bitfield,
    crate::{
        memory,
        x64,
    },
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
    __: u8,
    #[bits(5, access = RO)]
    zero: u8,
    #[bits(19)]
    __: u32,
}

impl Descriptor {
    pub fn base_address(&self) -> Option<usize> {
        let higher_base_address: usize = (self.base() as usize) << u32::BITS;
        let lower_descriptor: short::Descriptor = self.lower_descriptor();
        let lower_descriptor: Option<memory::segment::descriptor::Interface> = (&lower_descriptor).into();
        lower_descriptor.map(|lower_descriptor| lower_descriptor.base() + higher_base_address)
    }

    pub fn lower_descriptor(&self) -> short::Descriptor {
        self.descriptor().into()
    }
}

impl From<&descriptor::Interface> for Descriptor {
    fn from(interface: &descriptor::Interface) -> Self {
        let descriptor: short::Descriptor = interface.into();
        let descriptor: u64 = descriptor.into();
        let base: usize = interface.base();
        let base: u32 = (base >> u32::BITS) as u32;
        Self::default()
            .with_descriptor(descriptor)
            .with_base(base)
    }
}

impl From<&x64::task::state::segment::AndIoPermissionBitMap> for Descriptor {
    fn from(segment_and_io_permission_bit_map: &x64::task::state::segment::AndIoPermissionBitMap) -> Self {
        let descriptor: short::Descriptor = segment_and_io_permission_bit_map.into();
        let descriptor: u64 = descriptor.into();
        let base: *const x64::task::state::segment::AndIoPermissionBitMap = segment_and_io_permission_bit_map as *const x64::task::state::segment::AndIoPermissionBitMap;
        let base: u64 = base as u64;
        let base: u32 = (base >> u32::BITS) as u32;
        Self::default()
            .with_descriptor(descriptor)
            .with_base(base)
    }
}

