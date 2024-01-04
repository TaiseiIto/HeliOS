/// # EFI_DEVICE_PATH_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 10.2 EFI Device Path Protocol
#[derive(Debug)]
#[repr(C)]
pub struct DevicePath {
    base_type: u8,
    sub_type: u8,
    length: [u8; 2],
}

