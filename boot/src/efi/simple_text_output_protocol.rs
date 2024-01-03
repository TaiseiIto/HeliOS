/// # EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
#[derive(Debug)]
#[repr(C)]
pub struct SimpleTextOutputProtocol {
    reset: TextReset,
    output_string: TextString,
    test_string: TextTestString,
    query_mode: TextQueryMode,
}

/// # EFI_TEXT_RESET
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextReset = extern "efiapi" fn(&SimpleTextOutputProtocol, bool) -> super::Status;

/// # EFI_TEXT_STRING
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextString = extern "efiapi" fn(&SimpleTextOutputProtocol, super::char16::NullTerminatedString) -> super::Status;

/// # EFI_TEXT_TEST_STRING
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextTestString = extern "efiapi" fn(&SimpleTextOutputProtocol, super::char16::NullTerminatedString) -> super::Status;

/// # EFI_TEXT_QUERY_MODE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextQueryMode = extern "efiapi" fn(&SimpleTextOutputProtocol, usize, &mut usize, &mut usize) -> super::Status;

