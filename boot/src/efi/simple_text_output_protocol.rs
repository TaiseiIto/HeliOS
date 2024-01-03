/// # EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
#[derive(Debug)]
#[repr(C)]
pub struct SimpleTextOutputProtocol {
    reset: TextReset,
}

/// # EFI_TEXT_RESET
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
#[derive(Debug)]
#[repr(C)]
struct TextReset(extern "efiapi" fn(&SimpleTextOutputProtocol, bool) -> super::Status);

