use super::super::super::{
    Char16,
    Event,
    Status,
};

/// # EFI_SIMPLE_TEXT_INPUT_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.3 Simple Text Input Protocol
#[derive(Debug)]
#[repr(C)]
pub struct Protocol<'a> {
    reset: InputReset,
    read_key_stroke: InputReadKey,
    wait_for_key: Event<'a>,
}

/// # EFI_INPUT_RESET
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.3 Simple Text Input Protocol
type InputReset = extern "efiapi" fn(/* This */ &Protocol, /* ExtendedVerification */ bool) -> Status;

/// # EFI_INPUT_READ_KEY
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.3 Simple Text Input Protocol
type InputReadKey = extern "efiapi" fn(/* This */ &Protocol, /* Key */ &InputKey) -> Status;

#[repr(C)]
#[repr(C)]
struct InputKey {
    scan_code: u16,
    unicode_char: Char16,
}

