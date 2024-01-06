use {
    alloc::vec::Vec,
    super::{
        Event,
        Guid,
        Handle,
        Status,
        TableHeader,
        VOID,
        Void,
        char16,
        event,
        memory,
        protocol,
        time,
    },
};

/// # EFI_BOOT_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.4 EFI Boot Services Table
#[derive(Debug)]
#[repr(C)]
pub struct BootServices {
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
    reserved: *const Void,
    register_protocol_notify: RegisterProtocolNotify,
    locate_handle: LocateHandle,
    locate_device_path: LocateDevicePath,
    install_configuration_table: InstallConfigurationTable,
    load_image: ImageLoad,
    start_image: ImageStart,
    exit: Exit,
    unload_image: ImageUnload,
    exit_boot_services: ExitBootServices,
    get_next_monotonic_count: GetNextMonotonicCount,
    stall: Stall,
    set_watchdog_timer: SetWatchdogTimer,
    connect_controller: ConnectController,
    disconnect_controller: DisconnectController,
    open_protocol: OpenProtocol,
    close_protocol: CloseProtocol,
    open_protocol_information: OpenProtocolInformation,
    protocols_per_handle: ProtocolsPerHandle,
    locate_handle_buffer: LocateHandleBuffer,
    locate_protocol: LocateProtocol,
    install_multiple_protocol_interfaces: InstallMultipleProtocolInterfaces,
    uninstall_multiple_protocol_interfaces: UninstallMultipleProtocolInterfaces,
    calculate_crc32: CalculateCrc32,
    copy_mem: CopyMem,
    set_mem: SetMem,
    create_event_ex: CreateEventEx,
}

impl BootServices {
    pub fn allocate_pool(&self, size: usize) -> Result<&Void, Status> {
        let mut pool: &Void = &VOID;
        let result: Result<(), Status> = (self.allocate_pool)(memory::Type::LoaderData, size, &mut pool).into();
        result.map(|_| pool)
    }

    pub fn free_pool(&self, pool: &Void) -> Result<(), Status> {
        (self.free_pool)(pool).into()
    }

    pub fn memory_map(&self) -> Vec<memory::Descriptor> {
        let mut size: usize = 0;
        let descriptor: usize = 0;
        let descriptor: *mut memory::Descriptor = descriptor as *mut memory::Descriptor;
        let descriptor: &mut memory::Descriptor = unsafe {
            &mut *descriptor
        };
        let mut key: usize = 0;
        let mut descriptor_size: usize = 0;
        let mut descriptor_version: u32 = 0;
        let status: Result<(), Status> = (self.get_memory_map)(
            &mut size,
            descriptor,
            &mut key,
            &mut descriptor_size,
            &mut descriptor_version
        ).into();
        size += 2 * descriptor_size;
        let mut map: Vec<u8> = (0..size)
            .map(|_| 0)
            .collect();
        let descriptor: &mut u8 = &mut map[0];
        let descriptor: *mut u8 = descriptor as *mut u8;
        let descriptor: *mut memory::Descriptor = descriptor as *mut memory::Descriptor;
        let descriptor: &mut memory::Descriptor = unsafe {
            &mut *descriptor
        };
        let status: Result<(), Status> = (self.get_memory_map)(
            &mut size,
            descriptor,
            &mut key,
            &mut descriptor_size,
            &mut descriptor_version
        ).into();
        map[0..size]
            .chunks(descriptor_size)
            .map(|descriptor| {
                let descriptor: *const [u8] = descriptor as *const [u8];
                let descriptor: *const memory::Descriptor = descriptor as *const memory::Descriptor;
                let descriptor: &memory::Descriptor = unsafe {
                    &*descriptor
                };
                descriptor.clone()
            })
            .collect()
    }
}

/// # EFI_CREATE_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CreateEvent = extern "efiapi" fn(/* Type */ u32, /* NotifyTpl */ Tpl, /* NotifyFunction */ event::Notify, /* NotifyContext */ &Void, /* Event */ &mut Event) -> Status;

/// # EFI_TPL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type Tpl = usize;

/// # EFI_CREATE_EVENT_EX
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CreateEventEx = extern "efiapi" fn(/* Type */ u32, /* NotifyTpl */ Tpl, /* NotifyFunction */ event::Notify, /* NotifyContext */ &Void, /* EventGroup */ &Guid, /* Event */ &mut Event) -> Status;

/// # EFI_CLOSE_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CloseEvent = extern "efiapi" fn(/* Event */ Event) -> Status;

/// # EFI_SIGNAL_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type SignalEvent = extern "efiapi" fn(/* Event */ Event) -> Status;

/// # EFI_WAIT_FOR_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type WaitForEvent = extern "efiapi" fn(/* NumberOfEvents */ usize, /* Event */ &Event, /* Index */ &mut usize) -> Status;

/// # EFI_CHECK_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CheckEvent = extern "efiapi" fn(/* Event */ Event) -> Status;

/// # EFI_SET_TIMER
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type SetTimer = extern "efiapi" fn(/* Event */ Event, /* Type */ time::Delay, /*TriggerTime*/ u64) -> Status;

/// # EFI_RAISE_TPL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type RaiseTpl = extern "efiapi" fn(/* NewTpl */ Tpl) -> Tpl;

/// # EFI_RESTORE_TPL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type RestoreTpl = extern "efiapi" fn(/* OldTpl */ Tpl);

/// # EFI_ALLOCATE_PAGES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type AllocatePages = extern "efiapi" fn(/* Type */ memory::AllocateType, /* MemoryType */ memory::Type, /* Pages */ usize, /* Memory */ &mut memory::PhysicalAddress) -> Status;

/// # EFI_FREE_PAGES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type FreePages = extern "efiapi" fn(/* Memory */ memory::PhysicalAddress, /* Pages */ usize) -> Status;

/// # EFI_GET_MEMORY_MAP
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type GetMemoryMap = extern "efiapi" fn(/* MemoryMapSize */ &mut usize, /* MemoryMap */ &mut memory::Descriptor, /* MapKey */ &mut usize, /* DescriptorSize */ &mut usize, /* DescriptorVersion */ &mut u32) -> Status;

/// # EFI_ALLOCATE_POOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type AllocatePool = extern "efiapi" fn(/* PoolType */ memory::Type, /* Size */ usize, /* Buffer */ &mut &Void) -> Status;

/// # EFI_FREE_POOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type FreePool = extern "efiapi" fn(/* Buffer */ &Void) -> Status;

/// # EFI_INSTALL_PROTOCOL_INTERFACE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type InstallProtocolInterface = extern "efiapi" fn(/* Handle */ &mut Handle, /* Protocol */ &Guid, /* InterfaceType */ protocol::InterfaceType, /* Interface */ &Void) -> Status;

/// # EFI_UNINSTALL_PROTOCOL_INTERFACE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type UninstallProtocolInterface = extern "efiapi" fn(/* Handle */ Handle, /* Protocol */ &Guid, /* Interface */ &Void) -> Status;

/// # EFI_REINSTALL_PROTOCOL_INTERFACE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type ReinstallProtocolInterface = extern "efiapi" fn(/* Handle */ Handle, /* Protocol */ &Guid, /* OldInterface*/ &Void, /* NewInterface */ &Void) -> Status;

/// # EFI_REGISTER_PROTOCOL_NOTIFY
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type RegisterProtocolNotify = extern "efiapi" fn(/* Protocol */ &Guid, /* Event */ Event, /* Registration */ &mut &Void) -> Status;

/// # EFI_LOCATE_HANDLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type LocateHandle = extern "efiapi" fn(/* SearchType */ protocol::LocateSearchType, /* Protocol */ &Guid, /* SearchKey */ &Void, /* BufferSize */ &mut usize, /* Buffer */ &mut Handle) -> Status;

/// # EFI_HANDLE_HANDLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type HandleProtocol = extern "efiapi" fn(/* Handle */ Handle, /* Protocol */ &Guid, /* Interface */ &mut &Void) -> Status;

/// # EFI_LOCATE_DEVICE_PATH
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type LocateDevicePath = extern "efiapi" fn(/* Protocol */ &Guid, /* DevicePath */ &mut &protocol::DevicePath, /* Device */ &mut Handle) -> Status;

/// # EFI_OPEN_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type OpenProtocol = extern "efiapi" fn(/* Handle */ Handle, /* Protocol */ &Guid, /* Interface */ &mut &Void, /* AgentHandle */ Handle, /* ControllerHandle */ Handle, /* Attributes */ u32) -> Status;

/// # EFI_CLOSE_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type CloseProtocol = extern "efiapi" fn(/* Handle */ Handle, /* Protocol */ &Guid, /* AgentHandle */ Handle, /* ControllerHandle */ Handle) -> Status;

/// # EFI_OPEN_PROTOCOL_INFORMATION
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type OpenProtocolInformation = extern "efiapi" fn(/* Handle */ Handle, /* Protocol */ &Guid, /* EntryBuffer */ &mut &protocol::OpenProtocolInformationEntry, /* EntryCount */ &mut usize) -> Status;

/// # EFI_CONNECT_CONTROLLER
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type ConnectController = extern "efiapi" fn(/* ControllerHandle */ Handle, /* DriverImageHandle */ &Handle, /* RemainingDevicePath */ &protocol::DevicePath, /* Recursive */ bool) -> Status;

/// # EFI_DISCONNECT_CONTROLLER
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type DisconnectController = extern "efiapi" fn(/* ControllerHandle */ Handle, /* DriverImageHandle */ Handle, /* ChildHandle */ Handle) -> Status;

/// # EFI_PROTOCOLS_PER_HANDLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type ProtocolsPerHandle = extern "efiapi" fn(/* Handle */ Handle, /* ProtocolBuffer */ &mut &&Guid, /* ProtocolBufferCount */ &mut usize) -> Status;

/// # EFI_LOCATE_HANDLE_BUFFER
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type LocateHandleBuffer = extern "efiapi" fn(/* SearchType */ protocol::LocateSearchType, /* Protocol */ &Guid, /* SearchKey */ &Void, /* NoHandles */ &mut usize, /* Buffer */ &mut &Handle) -> Status;

/// # EFI_LOCATE_PROTOCOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type LocateProtocol = extern "efiapi" fn(/* Protocl */ &Guid, /* Registration */ &Void, /* Interface */&mut &Void) -> Status;

/// # EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type InstallMultipleProtocolInterfaces = extern "efiapi" fn(/* Handle */ &mut Handle) -> Status;

/// # EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.3 Protocol Handler Services
type UninstallMultipleProtocolInterfaces = extern "efiapi" fn(/* Handle */ &mut Handle) -> Status;

/// # EFI_IMAGE_LOAD
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ImageLoad = extern "efiapi" fn(/* BootPolicy */ bool, /* ParentImageHandle */ Handle, /* DevicePath */ &protocol::DevicePath, /* SourceBuffer */ &Void, /* SourceSize */ usize, /* ImageHandle */ &mut Handle) -> Status;

/// # EFI_IMAGE_START
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ImageStart = extern "efiapi" fn(/* ImageHandle */ Handle, /* ExitDataSize */ &mut usize, /* ExitData */ &mut char16::NullTerminatedString) -> Status;

/// # EFI_IMAGE_UNLOAD
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ImageUnload = extern "efiapi" fn(/* ImageHandle */ Handle) -> Status;

/// # EFI_EXIT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type Exit = extern "efiapi" fn(/* ImageHandle */ Handle, /* ExitStatus */ Status, /* ExitDataSize */ usize, /* ExitData */ char16::NullTerminatedString) -> Status;

/// # EFI_EXIT_BOOT_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.4 Image Services
type ExitBootServices = extern "efiapi" fn(/* ImageHandle */ Handle, /* MapKey */ usize) -> Status;

/// # EFI_SET_WATCHDOG_TIMER
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.5 Miscellaneous Boot Services
type SetWatchdogTimer = extern "efiapi" fn(/* Timeout */ usize, /* WatchdogCode */ u64, /* DataSize */ usize, /* WatchdogData */ char16::NullTerminatedString) -> Status;

/// # EFI_COPY_MEM
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.5 Miscellaneous Boot Services
type CopyMem = extern "efiapi" fn(/* Destination */ &Void, /* Source */ &Void, /* Length */ usize) -> Status;

/// # EFI_SET_MEM
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.5 Miscellaneous Boot Services
type SetMem = extern "efiapi" fn(/* Buffer */ &Void, /* Size */ usize, /* Value */ u8) -> Status;

/// # EFI_INSTALL_CONFIGURATION_TABLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.5 Miscellaneous Boot Services
type InstallConfigurationTable = extern "efiapi" fn(/* Guid */ &Guid, /* Table */ &Void) -> Status;

/// # EFI_STALL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.5 Miscellaneous Boot Services
type Stall = extern "efiapi" fn(/* Microseconds */ usize) -> Status;

/// # EFI_GET_NEXT_MONOTONIC_COUNT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.5 Miscellaneous Boot Services
type GetNextMonotonicCount = extern "efiapi" fn(/* Count */ &mut u64) -> Status;

/// # EFI_CALCULATE_CRC32
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.5 Miscellaneous Boot Services
type CalculateCrc32 = extern "efiapi" fn(/* Data */ &Void, /* DataSize */ usize, /* Crc32 */&mut u32) -> Status;

