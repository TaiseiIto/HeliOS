use super::{
    Guid,
    Status,
    TableHeader,
    Time,
    Void,
    VOID,
    char16,
    memory,
    time,
};

/// # EFI_RUNTIME_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.5 EFI Runtime Services Table
#[derive(Debug)]
#[repr(C)]
pub struct RuntimeServices {
    hdr: TableHeader,
    get_time: GetTime,
    set_time: SetTime,
    get_wakeup_time: GetWakeupTime,
    set_wakeup_time: SetWakeupTime,
    set_virtual_address_map: SetVirtualAddressMap,
    convert_pointer: ConvertPointer,
    get_variable: GetVariable,
    get_next_variable_name: GetNextVariableName,
    set_variable: SetVariable,
    get_next_high_monotonic_count: GetNextHighMonotonicCount,
    reset_system: ResetSystem,
    update_capsule: UpdateCapsule,
    query_capsule_capabilities: QueryCapsuleCapabilities,
    query_variable_info: QueryVariableInfo,
}

impl RuntimeServices {
    pub fn shutdown(&self) {
        let reset_type = ResetType::Shutdown;
        let status = Status::SUCCESS;
        let data_size: usize = 0;
        let data: Void = VOID;
        (self.reset_system)(ResetType::Shutdown, Status::SUCCESS, data_size, &data);
    }
}

/// # GetVariable
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.2 Variable Services
type GetVariable = extern "efiapi" fn(/* VariableName */ char16::NullTerminatedString, /* VendorGuid */ &Guid, /* Attributes */ &mut u32, /* DataSize */ &mut usize, /* Data */ &mut Void) -> Status;

/// # GetNextVariableName
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.2 Variable Services
type GetNextVariableName = extern "efiapi" fn(/* VariableNameSize */ &mut usize, /* VariableName */ char16::NullTerminatedString, /* VendorGuid */ &mut Guid) -> Status;

/// # SetVariable
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.2 Variable Services
type SetVariable = extern "efiapi" fn(/* VariableName */ char16::NullTerminatedString, /* VendorGuid */ &Guid, /* Attributes */ u32, /* DataSize */ usize, /* Data */ &Void) -> Status;

/// # QueryVariableInfo
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.2 Variable Services
type QueryVariableInfo = extern "efiapi" fn(/* Attributes */ u32, /* MaximumVariableStorageSize */ &mut u64, /* RemainingVariableStorageSize */ &mut u64, /* MaximumVariableSize */ &mut u64) -> Status;

/// # GetTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type GetTime = extern "efiapi" fn(/* Time */ &mut Time, /* Capabilities */ &mut time::Capabilities) -> Status;

/// # SetTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type SetTime = extern "efiapi" fn(/* Time */ &Time) -> Status;

/// # GetWakeupTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type GetWakeupTime = extern "efiapi" fn(/* Enabled */ &mut bool, /* Pending */ &mut bool, /* Time */ &mut Time) -> Status;

/// # SetWakeupTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type SetWakeupTime = extern "efiapi" fn(/* Enable */ bool, /* Time */ &Time) -> Status;

/// # SetVirtualAddressMap
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.4 Virtual Memory Services
type SetVirtualAddressMap = extern "efiapi" fn(/* MemoryMapSize */ usize, /* DescriptorSize */ usize, /* DescriptorVersion */ u32, /* VirtualMap */ &memory::Descriptor) -> Status;

/// # ConvertPointer
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.4 Virtual Memory Services
type ConvertPointer = extern "efiapi" fn(/* DebugDisposition */ usize, /* Address */ &&Void) -> Status;

/// # ResetSystem
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.5.1 Reset System
type ResetSystem = extern "efiapi" fn(/* ResetType */ ResetType, /* ResetStatus */ Status, /* DataSize */ usize, /* ResetData */ &Void);

/// # EFI_RESET_TYPE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.5.1 Reset System
#[derive(Debug)]
#[repr(C)]
enum ResetType {
    Cold,
    Warm,
    Shutdown,
    PlatformSpecific,
}

/// # GetNextHighMonotonicCount
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.5.2 Get Next High Monotonic Count
type GetNextHighMonotonicCount = extern "efiapi" fn(/* HighCount */ &mut u32) -> Status;

/// # UpdateCapsule
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.5.3 Update Capsule
type UpdateCapsule = extern "efiapi" fn(/* CapsuleHeaderArray */ &&CapsuleHeader, /* CapsuleCount */ usize, /* ScatterGatherList */ memory::PhysicalAddress) -> Status;

/// EFI_CAPSULE_HEADER
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.5.3 Update Capsule
#[repr(C)]
struct CapsuleHeader {
    length: u64,
    data_block_or_continuation_pointer: DataBlockOrContinuationPointer,
}

/// A union of data block or continuation pointer
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.5.3 Update Capsule
#[repr(C)]
union DataBlockOrContinuationPointer {
    data_block: memory::PhysicalAddress,
    continuation_pointer: memory::PhysicalAddress,
}

/// # QueryCapsuleCapabilities
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.5.3 Update Capsule
type QueryCapsuleCapabilities = extern "efiapi" fn(/* CapsuleHeaderArray */ &&CapsuleHeader, /* CapsuleCount */ usize, /* MaximumCapsuleSize */ &mut u64, /* ResetType */ &mut ResetType) -> Status;

