//! # MP Services Protocol
//! ## References
//! * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4 MP Services Protocol

use {
    core::fmt,
    super::super::{
        Event,
        Guid,
        Status,
        SystemTable,
        Void,
    },
};

/// # EFI_MP_SERVICES_PROTOCOL
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.1 EFI_MP_SERVICES_PROTOCOL
#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    get_number_of_processors: GetNumberOfProcessors,
    get_processor_info: GetProcessorInfo,
    startup_all_aps: StartupAllAps,
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

/// # EFI_MP_SERVICES_GET_PROCESSOR_INFO
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.3 EFI_MP_SERVICES_PROTOCOL.GetProcessorInfo()
type GetProcessorInfo = extern "efiapi" fn(/* This */ &Protocol, /* ProcessroNumber */ &usize, /* ProcessorInfoBuffer */ &mut ProcessorInformation) -> Status;

/// # EFI_PROCESSOR_INFORMATION
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.3 EFI_MP_SERVICES_PROTOCOL.GetProcessorInfo()
#[derive(Debug)]
#[repr(C)]
pub struct ProcessorInformation {
    processor_id: u64,
    status_flag: u32,
    location: CpuPhysicalLocation,
    extended_information: ExtendedProcessorInformation,
}

/// # EFI_CPU_PHYSICAL_LOCATION
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.3 EFI_MP_SERVICES_PROTOCOL.GetProcessorInfo()
#[derive(Debug)]
#[repr(C)]
pub struct CpuPhysicalLocation {
    package: u32,
    core: u32,
    thread: u32,
}

/// # EXTENDED_PROCESSOR_INFORMATION
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.3 EFI_MP_SERVICES_PROTOCOL.GetProcessorInfo()
#[repr(C)]
pub union ExtendedProcessorInformation {
    location2: CpuPhysicalLocation2,
}

impl ExtendedProcessorInformation {
    fn location2(&self) -> &CpuPhysicalLocation2 {
        unsafe {
            &self.location2
        }
    }
}

impl fmt::Debug for ExtendedProcessorInformation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ExtendedProcessorInformation")
            .field("location2", self.location2())
            .finish()
    }
}

/// # EFI_CPU_PHYSICAL_LOCATION2
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.3 EFI_MP_SERVICES_PROTOCOL.GetProcessorInfo()
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct CpuPhysicalLocation2 {
    package: u32,
    module: u32,
    tile: u32,
    die: u32,
    core: u32,
    thread: u32,
}

/// # EFI_MP_SERVICES_STARTUP_ALL_APS
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.4 EFI_MP_SERVICES_PROTOCOL.StartupAllAPs()
type StartupAllAps = extern "efiapi" fn(/* This */ &Protocol, /* Procedure */ ApProcedure, /* SingleThread */ bool, /* WaitEvent */ Event, /* TimeoutINMicroSeconds */ usize, /* ProcedureArgument */ &Void, /* FailedCpuList */ &mut &usize) -> Status;

/// # EFI_AP_PROCEDURE
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.4 EFI_MP_SERVICES_PROTOCOL.StartupAllAPs()
type ApProcedure = extern "efiapi" fn(/* ProcedureArgument */ &Void);

