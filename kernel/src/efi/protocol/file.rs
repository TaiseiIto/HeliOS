//! # File Protocol
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol

pub mod system;

use {
    alloc::{
        string::String,
        vec::Vec,
    },
    bitfield_struct::bitfield,
    super::super::{
        Char16,
        Event,
        Guid,
        Status,
        Time,
        Void,
        char16,
        null,
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

/// # EFI_FILE_INFO
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.5 File Protocol
#[derive(Debug)]
#[repr(C)]
pub struct Info {
    size: u64,
    file_size: u64,
    physical_size: u64,
    create_time: Time,
    last_access_time: Time,
    modification_time: Time,
    attributes: Attributes,
    file_name: Char16,
}

#[derive(Clone, Debug)]
pub struct Information {
    #[allow(dead_code)]
    size: u64,
    #[allow(dead_code)]
    file_size: u64,
    #[allow(dead_code)]
    physical_size: u64,
    #[allow(dead_code)]
    create_time: Time,
    #[allow(dead_code)]
    last_access_time: Time,
    #[allow(dead_code)]
    modification_time: Time,
    attributes: Attributes,
    file_name: String,
}

impl Information {
    fn is_directory(&self) -> bool {
        self.attributes.directory()
    }

    fn name(&self) -> &str {
        &self.file_name
    }
}

impl From<&Info> for Information {
    fn from(info: &Info) -> Self {
        let Info {
            size,
            file_size,
            physical_size,
            create_time,
            last_access_time,
            modification_time,
            attributes,
            file_name,
        } = info;
        let size: u64 = *size;
        let file_size: u64 = *file_size;
        let physical_size: u64 = *physical_size;
        let create_time: Time = create_time.clone();
        let last_access_time: Time = last_access_time.clone();
        let modification_time: Time = modification_time.clone();
        let attributes: Attributes = *attributes;
        let file_name: char16::NullTerminatedString = file_name.into();
        let file_name: String = (&file_name).into();
        Self {
            size,
            file_size,
            physical_size,
            create_time,
            last_access_time,
            modification_time,
            attributes,
            file_name,
        }
    }
}

impl From<&Protocol> for Information {
    fn from(protocol: &Protocol) -> Information {
        let information_type = Guid::new(0x09576e92, 0x6d3f, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);
        let mut buffer_size: usize = 0;
        let mut buffer = Void;
        let status: Status = (protocol.get_info)(protocol, &information_type, &mut buffer_size, &mut buffer)
            .result()
            .unwrap_err();
        assert!(status == Status::BUFFER_TOO_SMALL);
        let mut buffer: Vec<u8> = (0..buffer_size)
            .map(|_| 0)
            .collect();
        let buffer: &mut u8 = &mut buffer[0];
        let buffer: *mut u8 = buffer as *mut u8;
        let buffer: *mut Void = buffer as *mut Void;
        let buffer: &mut Void = unsafe {
            &mut *buffer
        };
        (protocol.get_info)(protocol, &information_type, &mut buffer_size, buffer)
            .result()
            .unwrap();
        let buffer: &Void = buffer;
        let buffer: *const Void = buffer as *const Void;
        let buffer: *const Info = buffer as *const Info;
        let buffer: &Info = unsafe {
            &*buffer
        };
        buffer.into()
    }
}

#[derive(Clone, Debug)]
pub struct Node<'a> {
    information: Information,
    protocol: &'a Protocol,
}

impl Node<'_> {
    pub fn name(&self) -> &str {
        self.information.name()
    }

    #[allow(dead_code)]
    pub fn read(&self) -> Vec<u8> {
        (!self.is_directory())
            .then(|| {
                let mut bytes: Vec<u8> = (0..self.information.file_size)
                    .map(|_| 0)
                    .collect();
                let buffer: &mut u8 = &mut bytes[0];
                let buffer: *mut u8 = buffer as *mut u8;
                let buffer: *mut Void = buffer as *mut Void;
                let buffer: &mut Void = unsafe {
                    &mut *buffer
                };
                let mut buffer_size: usize = bytes.len();
                (self.protocol.read)(self.protocol, &mut buffer_size, buffer)
                    .result()
                    .unwrap();
                bytes
            })
            .unwrap_or_default()
    }

    fn is_directory(&self) -> bool {
        self.information.is_directory()
    }
}

impl<'a> Iterator for Node<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.is_directory()
            .then(|| {
                let mut buffer_size: usize = 0;
                let mut buffer = Void;
                (self.protocol.read)(self.protocol, &mut buffer_size, &mut buffer)
                    .result()
                    .err()
                    .map(|status| (status, buffer_size))
            })
            .flatten()
            .and_then(|(status, mut buffer_size)| {
                assert!(status == Status::BUFFER_TOO_SMALL);
                let mut buffer: Vec<u8> = (0..buffer_size)
                    .map(|_| 0)
                    .collect();
                let buffer: &mut u8 = &mut buffer[0];
                let buffer: *mut u8 = buffer as *mut u8;
                let buffer: *mut Void = buffer as *mut Void;
                let buffer: &mut Void = unsafe {
                    &mut *buffer
                };
                (self.protocol.read)(self.protocol, &mut buffer_size, buffer)
                    .result()
                    .unwrap();
                (buffer_size != 0).then(|| {
                    let buffer: &Void = buffer;
                    let buffer: *const Void = buffer as *const Void;
                    let buffer: *const Info = buffer as *const Info;
                    let buffer: &Info = unsafe {
                        &*buffer
                    };
                    let buffer: Information = buffer.into();
                    buffer
                })
            })
            .map(|information| {
                let mut protocol: &Protocol = null();
                let file_name: Vec<u16> = char16::NullTerminatedString::string2vec(&information.file_name);
                let file_name: char16::NullTerminatedString = (&file_name).into();
                let open_mode = OpenMode::default()
                    .with_read(true);
                let attributes = Attributes::default();
                (self.protocol.open)(self.protocol, &mut protocol, file_name, open_mode, attributes)
                    .result()
                    .unwrap();
                Self {
                    information,
                    protocol,
                }
            })
    }
}

impl<'a> From<&'a Protocol> for Node<'a> {
    fn from(protocol: &'a Protocol) -> Self {
        let information: Information = protocol.into();
        Self {
            information,
            protocol,
        }
    }
}

