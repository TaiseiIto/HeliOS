/// # EFI_RUNTIME_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.5 EFI Runtime Services Table
#[derive(Debug)]
#[repr(C)]
pub struct RuntimeServices {
    hdr: super::TableHeader,
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
}

/// # GetTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type GetTime = extern "efiapi" fn(&mut Time, &mut TimeCapabilities) -> super::Status;

/// # EFI_TIME
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
#[repr(C)]
struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    pad1: u8,
    nanosecond: u32,
    time_zone: i16,
    day_light: u8,
    pad2: u8,
}

/// # EFI_TIME_CAPABILITIES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
#[repr(C)]
struct TimeCapabilities {
    resolution: u32,
    accuracy: u32,
    sets_to_zero: bool,
}

/// # SetTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type SetTime = extern "efiapi" fn(&Time) -> super::Status;

/// # GetWakeupTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type GetWakeupTime = extern "efiapi" fn(&mut bool, &mut bool, &mut Time) -> super::Status;

/// # SetWakeupTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type SetWakeupTime = extern "efiapi" fn(bool, &Time) -> super::Status;

/// # SetVirtualAddressMap
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.4 Virtual Memory Services
type SetVirtualAddressMap = extern "efiapi" fn(usize, usize, u32, &super::memory::Descriptor) -> super::Status;

/// # ConvertPointer
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.4 Virtual Memory Services
type ConvertPointer = extern "efiapi" fn(usize, &&()) -> super::Status;

/// # GetVariable
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.2 Variable Services
type GetVariable = extern "efiapi" fn(super::char16::NullTerminatedString, &super::Guid, &mut u32, &mut usize, &mut ()) -> super::Status;

/// # GetNextVariableName
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.2 Variable Services
type GetNextVariableName = extern "efiapi" fn(&mut usize, super::char16::NullTerminatedString, &mut super::Guid) -> super::Status;

/// # SetVariable
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.2 Variable Services
type SetVariable = extern "efiapi" fn(super::char16::NullTerminatedString, &super::Guid, u32, usize, &()) -> super::Status;

/// # ResetSystem
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.5.1 Reset System
type ResetSystem = extern "efiapi" fn(ResetType, super::Status, usize, &());

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
type GetNextHighMonotonicCount = extern "efiapi" fn(&mut u32) -> super::Status;

