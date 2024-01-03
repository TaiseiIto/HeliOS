/// # EFI_SIMPLE_TEXT_INPUT_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.3 Simple Text Input Protocol
#[derive(Debug)]
#[repr(C)]
pub struct SimpleTextInputProtocol {
    reset: InputReset,
}

/// # EFI_INPUT_RESET
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.3 Simple Text Input Protocol
#[derive(Debug)]
pub struct InputReset(pub extern "efiapi" fn(&SimpleTextInputProtocol, bool) -> super::Status);

