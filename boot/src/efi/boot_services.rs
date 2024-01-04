/// # EFI_BOOT_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.4 EFI Boot Services Table
#[derive(Debug)]
#[repr(C)]
pub struct BootServices<'a> {
    hdr: super::TableHeader,
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
    reserved: &'a super::Void,
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
type CreateEvent = extern "efiapi" fn(u32, Tpl, super::event::Notify, &super::Void, &mut super::Event) -> super::Status;

/// # EFI_CLOSE_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CloseEvent = extern "efiapi" fn(super::Event) -> super::Status;

/// # EFI_SIGNAL_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type SignalEvent = extern "efiapi" fn(super::Event) -> super::Status;

/// # EFI_WAIT_FOR_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type WaitForEvent = extern "efiapi" fn(usize, &super::Event, &mut usize) -> super::Status;

/// # EFI_CHECK_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CheckEvent = extern "efiapi" fn(super::Event) -> super::Status;

/// # EFI_SET_TIMER
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type SetTimer = extern "efiapi" fn(super::Event, super::time::Delay, u64) -> super::Status;

/// # EFI_ALLOCATE_PAGES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type AllocatePages = extern "efiapi" fn(super::memory::AllocateType, super::memory::Type, usize, &mut super::memory::PhysicalAddress) -> super::Status;

/// # EFI_FREE_PAGES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type FreePages = extern "efiapi" fn(super::memory::PhysicalAddress, usize) -> super::Status;

/// # EFI_GET_MEMORY_MAP
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type GetMemoryMap = extern "efiapi" fn(&mut usize, &mut super::memory::Descriptor, &mut usize, &mut usize, &mut u32) -> super::Status;

/// # EFI_ALLOCATE_POOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type AllocatePool = extern "efiapi" fn(super::memory::Type, usize, &mut &super::Void) -> super::Status;

/// # EFI_FREE_POOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type FreePool = extern "efiapi" fn(&super::Void) -> super::Status;

/// # EFI_INSTALL_PROTOCOL_INTERFACE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type InstallProtocolInterface = extern "efiapi" fn(&mut super::Handle, &super::Guid, InterfaceType, &super::Void) -> super::Status;

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
type UninstallProtocolInterface = extern "efiapi" fn(super::Handle, &super::Guid, &super::Void) -> super::Status;

/// # EFI_REINSTALL_PROTOCOL_INTERFACE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type ReinstallProtocolInterface = extern "efiapi" fn(super::Handle, &super::Guid, &super::Void, &super::Void) -> super::Status;

/// # EFI_REGISTER_PROTOCOL_NOTIFY
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type RegisterProtocolNotify = extern "efiapi" fn(&super::Guid, super::Event, &mut &super::Void) -> super::Status;

/// # EFI_LOCATE_HANDLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type LocateHandle = extern "efiapi" fn(LocateSearchType, &super::Guid, &super::Void, &mut usize, &mut super::Handle) -> super::Status;

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
type HandleProtocol = extern "efiapi" fn(super::Handle, &super::Guid, &mut &super::Void) -> super::Status;

/// # EFI_LOCATE_DEVICE_PATH
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type LocateDevicePath = extern "efiapi" fn(&super::Guid, &mut &super::protocol::DevicePath, &mut super::Handle) -> super::Status;

/// # EFI_IMAGE_LOAD
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ImageLoad = extern "efiapi" fn(bool, super::Handle, &super::protocol::DevicePath, &super::Void, usize, &mut super::Handle) -> super::Status;

/// # EFI_IMAGE_START
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ImageStart = extern "efiapi" fn(super::Handle, &mut usize, &mut super::char16::NullTerminatedString) -> super::Status;

/// # EFI_EXIT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type Exit = extern "efiapi" fn(super::Handle, super::Status, usize, super::char16::NullTerminatedString) -> super::Status;

/// # EFI_IMAGE_UNLOAD
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ImageUnload = extern "efiapi" fn(super::Handle) -> super::Status;

/// # EFI_EXIT_BOOT_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ExitBootServices = extern "efiapi" fn(super::Handle, usize) -> super::Status;

/// # EFI_INSTALL_CONFIGURATION_TABLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.5 Miscellaneous Boot Services
type InstallConfigurationTable = extern "efiapi" fn(&super::Guid, &super::Void) -> super::Status;

