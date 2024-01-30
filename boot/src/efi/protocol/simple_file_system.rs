//! # Simple File System Protocol
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.4 Simple File System Protocol

use super::{
    file,
    super::{
        Guid,
        Status,
        SystemTable,
        Void,
        null,
    },
};

/// # EFI_SIMPLE_FILE_SYSTEM_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.4 Simple File System Protocol
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    revision: u64,
    open_volume: OpenVolume,
}

impl Protocol {
    pub fn get() -> &'static Self {
        let guid = Guid::new(0x964e5b22, 0x6459, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);
        let registration: &Void = null();
        let protocol: &Void = SystemTable::get()
            .locate_protocol(registration, guid)
            .unwrap();
        let protocol: *const Void = protocol as *const Void;
        let protocol: *const Protocol = protocol as *const Protocol;
        unsafe {
            &*protocol
        }
    }

    pub fn open_volume(&self) -> Result<&file::Protocol, Status> {
        let mut root: &file::Protocol = null();
        (self.open_volume)(self, &mut root)
            .result()
            .map(|_| root)
    }
}

/// # EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 13.4 Simple File System Protocol
type OpenVolume = extern "efiapi" fn(/* This */ &Protocol, &mut &file::Protocol) -> Status;

