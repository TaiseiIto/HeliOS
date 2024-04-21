//! # MP Services Protocol
//! ## References
//! * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4 MP Services Protocol

use {
    alloc::collections::BTreeMap,
    core::fmt,
    super::super::{
        Event,
        Guid,
        Status,
        SystemTable,
        Void,
        null,
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
    startup_this_ap: StartupThisAp,
    switch_bsp: SwitchBsp,
    enable_disable_ap: EnableDisableAp,
    who_am_i: WhoAmI,
}

impl Protocol {
    #[allow(dead_code)]
    pub fn get<'a>(system_table: &'a SystemTable<'a>) -> &'a Self {
        let guid = Guid::new(0x3fdda605, 0xa76e, 0x4f46, [0xad, 0x29, 0x12, 0xf4, 0x53, 0x1b, 0x3d, 0x08]);
        let registration: &Void = null();
        let protocol: &Void = system_table
            .locate_protocol(registration, guid)
            .unwrap();
        let protocol: *const Void = protocol as *const Void;
        let protocol: *const Protocol = protocol as *const Protocol;
        unsafe {
            &*protocol
        }
    }

    #[allow(dead_code)]
    pub fn get_all_processor_informations(&self) -> BTreeMap<usize, ProcessorInformation> {
        let number_of_processors: usize = self
            .number_of_processors()
            .unwrap()
            .all;
        (0..number_of_processors)
            .map(|processor_number| {
                let processor_information: ProcessorInformation = self
                    .get_processor_information(processor_number)
                    .unwrap();
                (processor_number, processor_information)
            })
            .collect()
    }

    #[allow(dead_code)]
    pub fn get_processor_information(&self, processor_number: usize) -> Result<ProcessorInformation, Status> {
        let mut processor_information = ProcessorInformation::default();
        (self.get_processor_info)(self, processor_number, &mut processor_information)
            .result()
            .map(|_| processor_information)
    }

    #[allow(dead_code)]
    pub fn my_processor_number(&self) -> Result<usize, Status> {
        let mut my_processor_number: usize = 0;
        (self.who_am_i)(self, &mut my_processor_number)
            .result()
            .map(|_| my_processor_number)
    }

    #[allow(dead_code)]
    pub fn number_of_processors(&self) -> Result<NumberOfProcessors, Status> {
        let mut all: usize = 0;
        let mut enabled: usize = 0;
        (self.get_number_of_processors)(self, &mut all, &mut enabled)
            .result()
            .map(|_| NumberOfProcessors {
                all,
                enabled,
            })
    }
}

/// # EFI_MP_SERVICES_GET_NUMBER_OF_PROCESSORS
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.2 EFI_MP_SERVICES_PROTOCOL.GetNumberOfProcessors()
type GetNumberOfProcessors = extern "efiapi" fn(/* This */ &Protocol, /* NumberOfProcessors */ &mut usize, /* NumberOfEnabledProcessors */ &mut usize) -> Status;

#[derive(Debug)]
pub struct NumberOfProcessors {
    #[allow(dead_code)]
    all: usize,
    #[allow(dead_code)]
    enabled: usize,
}

/// # EFI_MP_SERVICES_GET_PROCESSOR_INFO
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.3 EFI_MP_SERVICES_PROTOCOL.GetProcessorInfo()
type GetProcessorInfo = extern "efiapi" fn(/* This */ &Protocol, /* ProcessroNumber */ usize, /* ProcessorInfoBuffer */ &mut ProcessorInformation) -> Status;

/// # EFI_PROCESSOR_INFORMATION
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.3 EFI_MP_SERVICES_PROTOCOL.GetProcessorInfo()
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct ProcessorInformation {
    processor_id: u64,
    status_flag: u32,
    location: CpuPhysicalLocation,
    extended_information: ExtendedProcessorInformation,
}

impl ProcessorInformation {
    #[allow(dead_code)]
    pub fn identifier(&self) -> u64 {
        self.processor_id
    }
}

/// # EFI_CPU_PHYSICAL_LOCATION
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.3 EFI_MP_SERVICES_PROTOCOL.GetProcessorInfo()
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct CpuPhysicalLocation {
    package: u32,
    core: u32,
    thread: u32,
}

/// # EXTENDED_PROCESSOR_INFORMATION
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.3 EFI_MP_SERVICES_PROTOCOL.GetProcessorInfo()
#[derive(Clone, Copy)]
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

impl Default for ExtendedProcessorInformation {
    fn default() -> Self {
        let location2 = CpuPhysicalLocation2::default();
        Self {
            location2
        }
    }
}

/// # EFI_CPU_PHYSICAL_LOCATION2
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.3 EFI_MP_SERVICES_PROTOCOL.GetProcessorInfo()
#[derive(Clone, Copy, Debug, Default)]
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
type StartupAllAps = extern "efiapi" fn(/* This */ &Protocol, /* Procedure */ ApProcedure, /* SingleThread */ bool, /* WaitEvent */ Event, /* TimeoutInMicroSeconds */ usize, /* ProcedureArgument */ &Void, /* FailedCpuList */ &mut &usize) -> Status;

/// # EFI_AP_PROCEDURE
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.4 EFI_MP_SERVICES_PROTOCOL.StartupAllAPs()
type ApProcedure = extern "efiapi" fn(/* ProcedureArgument */ &Void);

/// # EFI_MP_SERVICES_STARTUP_THIS_AP
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.5 EFI_MP_SERVICES_PROTOCOL.StartupThisAP()
type StartupThisAp = extern "efiapi" fn(/* This */ &Protocol, /* Procedure */ ApProcedure, /* ProcessorNumber */ usize, /* WaitEvent */ Event, /* TimeoutInMicroSeconds */ usize, /* ProcedureArgument */ &Void, /* Finished */ &mut bool) -> Status;

/// # EFI_MP_SERVICES_SWITCH_BSP
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.6 EFI_MP_SERVICES_PROTOCOL.SwitchBSP()
type SwitchBsp = extern "efiapi" fn(/* This */ &Protocol, /* ProcessorNumber */ usize, /* EnableOldBSP */ bool) -> Status;

/// # EFI_MP_SERVICES_ENABLEDISABLEAP
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.7 EFI_MP_SERVICES_PROTOCOL.EnableDisableAp()
type EnableDisableAp = extern "efiapi" fn(/* This */ &Protocol, /* ProcessorNumber */ usize, /* EnableAP */ bool, /* HealthFlag */ &u32) -> Status;

/// # EFI_MP_SERVICES_WHOAMI
/// ## References
/// * [UEFI Platform Initialization Specification](https://uefi.org/sites/default/files/resources/UEFI_PI_Spec_1_8_March3.pdf) II-13.4.8 EFI_MP_SERVICES_PROTOCOL.WhoAmI()
type WhoAmI = extern "efiapi" fn(/* This */ &Protocol, /* ProcessorNumber */ &mut usize) -> Status;

