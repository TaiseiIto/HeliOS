use super::{
    Guid,
    Void,
};

/// # EFI_CONFIGURATION_TABLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.6 EFI Configuration Table & Properties Table
#[derive(Debug)]
#[repr(C)]
pub struct Table<'a> {
    vendor_guid: Guid,
    vendor_table: &'a Void,
}

