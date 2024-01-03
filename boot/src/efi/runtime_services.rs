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
    update_capsule: UpdateCapsule,
}

/// # GetTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type GetTime = extern "efiapi" fn(&mut super::Time, &mut super::time::Capabilities) -> super::Status;

/// # SetTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type SetTime = extern "efiapi" fn(&super::Time) -> super::Status;

/// # GetWakeupTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type GetWakeupTime = extern "efiapi" fn(&mut bool, &mut bool, &mut super::Time) -> super::Status;

/// # SetWakeupTime
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.3 Time Services
type SetWakeupTime = extern "efiapi" fn(bool, &super::Time) -> super::Status;

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

/// # UpdateCapsule
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 8.5.3 Update Capsule
type UpdateCapsule = extern "efiapi" fn(&&CapsuleHeader, usize, super::memory::PhysicalAddress) -> super::Status;

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
    data_block: super::memory::PhysicalAddress,
    continuation_pointer: super::memory::PhysicalAddress,
}

