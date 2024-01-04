/// # EFI_BOOT_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.4 EFI Boot Services Table
#[derive(Debug)]
#[repr(C)]
pub struct BootServices {
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
type AllocatePool = extern "efiapi" fn(super::memory::Type, usize, &mut &mut ()) -> super::Status;

/// # EFI_FREE_POOL
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.2 Memory Allocation Services
type FreePool = extern "efiapi" fn(&()) -> super::Status;

/// # EFI_CREATE_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CreateEvent = extern "efiapi" fn(u32, Tpl, EventNotify, &(), &mut Event) -> super::Status;

/// # EFI_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type Event<'a> = &'a ();

/// # EFI_EVENT_NOTIFY
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type EventNotify = extern "efiapi" fn(Event, &());

/// # EFI_CLOSE_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CloseEvent = extern "efiapi" fn(Event) -> super::Status;

/// # EFI_SIGNAL_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type SignalEvent = extern "efiapi" fn(Event) -> super::Status;

/// # EFI_WAIT_FOR_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type WaitForEvent = extern "efiapi" fn(usize, &Event, &mut usize) -> super::Status;

/// # EFI_CHECK_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type CheckEvent = extern "efiapi" fn(Event) -> super::Status;

/// # EFI_SET_TIMER
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
type SetTimer = extern "efiapi" fn(Event, super::time::Delay, u64) -> super::Status;
