use {
    bitfield_struct::bitfield,
    crate::memory,
    super::AndIoPermissionBitMap,
};

/// # TSS Descriptor in 64-Bit mode
/// ## References
/// * [Intel 64 and IA-32 Architectures Software Developer's Manual December 2023](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) Vol.3A 8.2.3 TSS Descriptor in 64-Bit mode
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
        let lower_descriptor: memory::segment::Descriptor = self.lower_descriptor();
        let lower_descriptor: Option<memory::segment::descriptor::Interface> = (&lower_descriptor).into();
        lower_descriptor.map(|lower_descriptor| lower_descriptor.base_address() + higher_base_address)
    }

    pub fn lower_descriptor(&self) -> memory::segment::Descriptor {
        self.descriptor().into()
    }
}

impl From<&AndIoPermissionBitMap> for Descriptor {
    fn from(segment_and_io_permission_bit_map: &AndIoPermissionBitMap) -> Self {
        let descriptor: memory::segment::Descriptor = segment_and_io_permission_bit_map.into();
        let descriptor: u64 = descriptor.into();
        let base: *const AndIoPermissionBitMap = segment_and_io_permission_bit_map as *const AndIoPermissionBitMap;
        let base: u64 = base as u64;
        let base: u32 = (base >> u32::BITS) as u32;
        Self::default()
            .with_descriptor(descriptor)
            .with_base(base)
    }
}

