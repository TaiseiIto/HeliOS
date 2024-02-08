use core::{
    fmt,
    iter,
};

/// # EFI_TABLE_HEADER
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.2 EFI Table Header
#[derive(Debug)]
#[repr(C)]
pub struct TableHeader {
    signature: Signature,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}

#[repr(C)]
struct Signature(u64);

impl fmt::Debug for Signature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        iter::once('"')
            .chain(self.0
                .to_le_bytes()
                .into_iter()
                .filter_map(|byte| char::from_u32(byte as u32)))
            .chain(iter::once('"'))
            .fold(Ok(()), |result, character| result.and(write!(formatter, "{}", character)))
    }
}

