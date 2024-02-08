use {
    core::fmt,
    super::Void,
};

/// # EFI_EVENT
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 2.3.1 Data Types
#[repr(C)]
pub struct Event<'a>(&'a super::Void);

impl fmt::Debug for Event<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#x?}", self.0 as *const Void)
    }
}

/// # EFI_EVENT_NOTIFY
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 7.1 Event, Timer, and Task Priority Services
pub type Notify = extern "efiapi" fn(/* Event */ Event, /* Context */ &super::Void);

