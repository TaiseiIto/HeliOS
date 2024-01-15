//! # MP Services Protocol
//! ## References
//! * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4 MP Services Protocol

use super::super::{
    Guid,
    Status,
    SystemTable,
    Void,
};

/// # EFI_MP_SERVICES_PROTOCOL
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.1 EFI_MP_SERVICES_PROTOCOL
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    get_number_of_processors: GetNumberOfProcessors,
}

impl Protocol {
    pub fn get() -> &'static Self {
        let guid = Guid::new(0x3fdda605, 0xa76e, 0x4f46, [0xad, 0x29, 0x12, 0xf4, 0x53, 0x1b, 0x3d, 0x08]);
        let registration: usize = 0;
        let registration: *const Void = registration as *const Void;
        let registration: &Void = unsafe {
            &*registration
        };
        let protocol: &Void = SystemTable::get()
            .locate_protocol(registration, guid)
            .unwrap();
        let protocol: *const Void = protocol as *const Void;
        let protocol: *const Protocol = protocol as *const Protocol;
        unsafe {
            &*protocol
        }
    }
}

/// # EFI_MP_SERVICES_GET_NUMBER_OF_PROCESSORS
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.2 EFI_MP_SERVICES_PROTOCOL.GetNumberOfProcessors()
type GetNumberOfProcessors = extern "efiapi" fn(/* This */ &Protocol, /* NumberOfProcessors */ &mut usize, /* NumberOfEnabledProcessors */ &mut usize) -> Status;

