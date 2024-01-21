pub mod database;
pub mod font;
pub mod graphics_output;
pub mod mp_services;
pub mod simple_text;

use super::Handle;

/// # EFI_INTERFACE_TYPE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub enum InterfaceType {
    NativeInterface,
}

/// # EFI_LOCATE_SEARCH_TYPE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
#[allow(dead_code)]
#[derive(Debug)]
#[repr(C)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

/// # EFI_OPEN_PROTOCOL_INFORMATION
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
#[derive(Debug)]
#[repr(C)]
pub struct OpenProtocolInformationEntry<'a> {
    agent_handle: Handle<'a>,
    controller_handle: Handle<'a>,
    attributes: u32,
    open_count: u32,
}

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

