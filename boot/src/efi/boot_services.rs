use super::{
    Event,
    Guid,
    Handle,
    Status,
    TableHeader,
    Void,
    char16,
    event,
    memory,
    protocol,
    time,
};

/// # EFI_BOOT_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.4 EFI Boot Services Table
#[derive(Debug)]
#[repr(C)]
pub struct BootServices<'a> {
    hdr: TableHeader,
    raise_tpl: RaiseTpl,
    restore_tpl: RestoreTpl,
    allocate_pages: AllocatePages,
    free_pages: FreePages,
    get_memory_map: GetMemoryMap,
    allocate_pool: AllocatePool,
    free_pool: FreePool,
    create_event: CreateEvent,
    set_timer: SetTimer,
    wait_for_event: WaitForEvent,
    signal_event: SignalEvent,
    close_event: CloseEvent,
    check_event: CheckEvent,
    install_protocol_interface: InstallProtocolInterface,
    reinstall_protocol_interface: ReinstallProtocolInterface,
    uninstall_protocol_interface: UninstallProtocolInterface,
    handle_protocol: HandleProtocol,
    reserved: &'a Void,
    register_protocol_notify: RegisterProtocolNotify,
    locate_handle: LocateHandle,
    locate_device_path: LocateDevicePath,
    install_configuration_table: InstallConfigurationTable,
    load_image: ImageLoad,
    start_image: ImageStart,
    exit: Exit,
    unload_image: ImageUnload,
    exit_boot_services: ExitBootServices,
}

/// # EFI_RAISE_TPL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type RaiseTpl = extern "efiapi" fn(Tpl) -> Tpl;

/// # EFI_RESTORE_TPL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type RestoreTpl = extern "efiapi" fn(Tpl);

/// # EFI_TPL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type Tpl = usize;

/// # EFI_CREATE_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CreateEvent = extern "efiapi" fn(u32, Tpl, event::Notify, &Void, &mut Event) -> Status;

/// # EFI_CLOSE_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CloseEvent = extern "efiapi" fn(Event) -> Status;

/// # EFI_SIGNAL_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type SignalEvent = extern "efiapi" fn(Event) -> Status;

/// # EFI_WAIT_FOR_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type WaitForEvent = extern "efiapi" fn(usize, &Event, &mut usize) -> Status;

/// # EFI_CHECK_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CheckEvent = extern "efiapi" fn(Event) -> Status;

/// # EFI_SET_TIMER
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type SetTimer = extern "efiapi" fn(Event, time::Delay, u64) -> Status;

/// # EFI_ALLOCATE_PAGES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type AllocatePages = extern "efiapi" fn(memory::AllocateType, memory::Type, usize, &mut memory::PhysicalAddress) -> Status;

/// # EFI_FREE_PAGES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type FreePages = extern "efiapi" fn(memory::PhysicalAddress, usize) -> Status;

/// # EFI_GET_MEMORY_MAP
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type GetMemoryMap = extern "efiapi" fn(&mut usize, &mut memory::Descriptor, &mut usize, &mut usize, &mut u32) -> Status;

/// # EFI_ALLOCATE_POOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type AllocatePool = extern "efiapi" fn(memory::Type, usize, &mut &Void) -> Status;

/// # EFI_FREE_POOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type FreePool = extern "efiapi" fn(&Void) -> Status;

/// # EFI_INSTALL_PROTOCOL_INTERFACE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type InstallProtocolInterface = extern "efiapi" fn(&mut Handle, &Guid, InterfaceType, &Void) -> Status;

/// # EFI_INTERFACE_TYPE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
#[derive(Debug)]
#[repr(C)]
pub enum InterfaceType {
    NativeInterface,
}

/// # EFI_UNINSTALL_PROTOCOL_INTERFACE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type UninstallProtocolInterface = extern "efiapi" fn(Handle, &Guid, &Void) -> Status;

/// # EFI_REINSTALL_PROTOCOL_INTERFACE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type ReinstallProtocolInterface = extern "efiapi" fn(Handle, &Guid, &Void, &Void) -> Status;

/// # EFI_REGISTER_PROTOCOL_NOTIFY
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type RegisterProtocolNotify = extern "efiapi" fn(&Guid, Event, &mut &Void) -> Status;

/// # EFI_LOCATE_HANDLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type LocateHandle = extern "efiapi" fn(LocateSearchType, &Guid, &Void, &mut usize, &mut Handle) -> Status;

/// # EFI_LOCATE_SEARCH_TYPE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
#[derive(Debug)]
#[repr(C)]
pub enum LocateSearchType {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}

/// # EFI_HANDLE_HANDLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type HandleProtocol = extern "efiapi" fn(Handle, &Guid, &mut &Void) -> Status;

/// # EFI_LOCATE_DEVICE_PATH
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type LocateDevicePath = extern "efiapi" fn(&Guid, &mut &protocol::DevicePath, &mut Handle) -> Status;

/// # EFI_IMAGE_LOAD
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ImageLoad = extern "efiapi" fn(bool, Handle, &protocol::DevicePath, &Void, usize, &mut Handle) -> Status;

/// # EFI_IMAGE_START
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ImageStart = extern "efiapi" fn(Handle, &mut usize, &mut char16::NullTerminatedString) -> Status;

/// # EFI_EXIT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type Exit = extern "efiapi" fn(Handle, Status, usize, char16::NullTerminatedString) -> Status;

/// # EFI_IMAGE_UNLOAD
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ImageUnload = extern "efiapi" fn(Handle) -> Status;

/// # EFI_EXIT_BOOT_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ExitBootServices = extern "efiapi" fn(Handle, usize) -> Status;

/// # EFI_INSTALL_CONFIGURATION_TABLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.5 Miscellaneous Boot Services
type InstallConfigurationTable = extern "efiapi" fn(&Guid, &Void) -> Status;

