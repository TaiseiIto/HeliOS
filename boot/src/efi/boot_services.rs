/// # EFI_BOOT_SERVICES
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.4 EFI Boot Services Table
#[derive(Debug)]
#[repr(C)]
pub struct BootServices {
    hdr: super::TableHeader,
    raise_tpl: RaiseTpl,
    restore_tpl: RestoreTpl,
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

