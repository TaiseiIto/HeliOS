use super::{
    Status,
    char16,
};

/// # EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
#[derive(Debug)]
#[repr(C)]
pub struct SimpleTextOutputProtocol<'a> {
    reset: TextReset,
    output_string: TextString,
    test_string: TextTestString,
    query_mode: TextQueryMode,
    set_mode: TextSetMode,
    set_attribute: TextSetAttribute,
    clear_screen: TextClearScreen,
    set_cursor_position: TextSetCursorPosition,
    enable_cursor: TextEnableCursor,
    mode: &'a SimpleTextOutputMode,
}

impl SimpleTextOutputProtocol<'_> {
    pub fn output_string(&self, string: char16::NullTerminatedString) -> Result<(), Status> {
        (self.output_string)(self, string).into()
    }
}

/// # EFI_TEXT_RESET
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextReset = extern "efiapi" fn(/* This */ &SimpleTextOutputProtocol, /* ExtendedVerification */ bool) -> Status;

/// # EFI_TEXT_STRING
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextString = extern "efiapi" fn(/* This */ &SimpleTextOutputProtocol, /* String */ char16::NullTerminatedString) -> Status;

/// # EFI_TEXT_TEST_STRING
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextTestString = extern "efiapi" fn(/* This */ &SimpleTextOutputProtocol, /* String */ char16::NullTerminatedString) -> Status;

/// # EFI_TEXT_QUERY_MODE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextQueryMode = extern "efiapi" fn(/* This */ &SimpleTextOutputProtocol, /* ModeNumber */ usize, /* Columns */ &mut usize, /* Rows */ &mut usize) -> Status;

/// # EFI_TEXT_SET_MODE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextSetMode = extern "efiapi" fn(/* This */ &SimpleTextOutputProtocol, /* ModeNumber */ usize) -> Status;

/// # EFI_TEXT_SET_ATTRIBUTE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextSetAttribute = extern "efiapi" fn(/* This */ &SimpleTextOutputProtocol, /* Attribute */ usize) -> Status;

/// # EFI_TEXT_CLEAR_SCREEN
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextClearScreen = extern "efiapi" fn(/* This */ &SimpleTextOutputProtocol) -> Status;

/// # EFI_TEXT_SET_CURSOR_POSITION
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextSetCursorPosition = extern "efiapi" fn(/* This */ &SimpleTextOutputProtocol, /* Column */ usize, /* Row */ usize) -> Status;

/// # EFI_TEXT_ENABLE_CURSOR
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
type TextEnableCursor = extern "efiapi" fn(/* This */ &SimpleTextOutputProtocol, /* Visible */ bool) -> Status;

/// # SIMPLE_TEXT_OUTPUT_MODE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 12.4 Simple Text Output Protocol
#[derive(Debug)]
#[repr(C)]
struct SimpleTextOutputMode {
    max_mode: i32,
    mode: i32,
    attribute: i32,
    cursor_column: i32,
    cursor_row: i32,
    cursor_visible: bool,
}

