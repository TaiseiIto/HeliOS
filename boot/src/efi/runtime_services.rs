/// # EFI_RUNTIME_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.5 EFI Runtime Services Table
#[derive(Debug)]
#[repr(C)]
pub struct RuntimeServices {
    hdr: super::TableHeader,
}

