use super::Void;

/// # EFI_HII_HANDLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 34.8 Database Protocol
pub type Handle<'a> = &'a Void;
