//! # File Protocol
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol

use {
    bitfield_struct::bitfield,
    super::super::{
        Status,
        Void,
        char16,
    },
};

/// # EFI_FILE_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    revision: u64,
    open: Open,
    close: Close,
    delete: Delete,
    read: Read,
    write: Write,
}

/// # EFI_FILE_OPEN
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type Open = extern "efiapi" fn(/* This */ &Protocol, /* NewHandle */ &mut &Protocol, /* FileName */ char16::NullTerminatedString, /* OpenMode */OpenMode, /* Attributes */ Attributes) -> Status;

/// # OpenMode
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
#[bitfield(u64)]
pub struct OpenMode {
    read: bool,
    write: bool,
    #[bits(62, access = RO)]
    reserved0: u64,
}

/// # Attributes
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
#[bitfield(u64)]
pub struct Attributes {
    read_only: bool,
    hidden: bool,
    system: bool,
    #[bits(access = RO)]
    reserved0: bool,
    directory: bool,
    archive: bool,
    #[bits(58, access = RO)]
    reserved1: u64,
}

/// # EFI_FILE_CLOSE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type Close = extern "efiapi" fn(/* This */ &Protocol) -> Status;

/// # EFI_FILE_DELETE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type Delete = extern "efiapi" fn(/* This */ &Protocol) -> Status;

/// # EFI_FILE_READ
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type Read = extern "efiapi" fn(/* This */ &Protocol, /* BufferSize */ &mut usize, /* Buffer */ &mut Void) -> Status;

/// # EFI_FILE_WRITE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type Write = extern "efiapi" fn(/* This */ &Protocol, /* BufferSize */ &mut usize, /* Buffer */ &Void) -> Status;

