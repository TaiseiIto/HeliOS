//! # File Protocol
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol

use {
    bitfield_struct::bitfield,
    super::super::{
        Event,
        Guid,
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
    get_position: GetPosition,
    set_position: SetPosition,
    get_info: GetInfo,
    set_info: SetInfo,
    flush: Flush,
    open_ex: OpenEx,
    read_ex: ReadEx,
    write_ex: WriteEx,
    flush_ex: FlushEx,
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

/// # EFI_FILE_OPEN_EX
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type OpenEx = extern "efiapi" fn(/* This */ &Protocol, /* NewHandle */ &mut &Protocol, /* FileName */ char16::NullTerminatedString, /* OpenMode */OpenMode, /* Attributes */ Attributes, /* Token */ &mut IoToken) -> Status;

/// # EFI_FILE_IO_TOKEN
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
#[derive(Debug)]
#[repr(C)]
pub struct IoToken<'a> {
    event: Event<'a>,
    status: Status,
    buffer_size: usize,
    buffer: &'a Void,
}

/// # EFI_FILE_READ_EX
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type ReadEx = extern "efiapi" fn(/* This */ &Protocol, /* Token */ &mut IoToken) -> Status;

/// # EFI_FILE_WRITE_EX
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type WriteEx = extern "efiapi" fn(/* This */ &Protocol, /* Token */ &mut IoToken) -> Status;

/// # EFI_FILE_FLUSH_EX
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type FlushEx = extern "efiapi" fn(/* This */ &Protocol, /* Token */ &mut IoToken) -> Status;

/// # EFI_FILE_SET_POSITION
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type SetPosition = extern "efiapi" fn(/* This */ &Protocol, /* Position */ u64) -> Status;

/// # EFI_FILE_GET_POSITION
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type GetPosition = extern "efiapi" fn(/* This */ &Protocol, /* Position */ &mut u64) -> Status;

/// # EFI_FILE_GET_INFO
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type GetInfo = extern "efiapi" fn(/* This */ &Protocol, /* InformationType */ &Guid, /* BufferSize */ &mut usize, /* Buffer */ &mut Void) -> Status;

/// # EFI_FILE_SET_INFO
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type SetInfo = extern "efiapi" fn(/* This */ &Protocol, /* InformationType */ &Guid, /* BufferSize */ usize, /* Buffer */ &Void) -> Status;

/// # EFI_FILE_FLUSH
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
type Flush = extern "efiapi" fn(/* This */ &Protocol) -> Status;

