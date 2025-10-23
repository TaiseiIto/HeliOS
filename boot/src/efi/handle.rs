use {super::Void, core::fmt};

/// # EFI_HANDLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 2.3.1 Data Types
#[repr(C)]
pub struct Handle<'a>(&'a Void);

impl fmt::Debug for Handle<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let void: *const Void = self.0 as *const Void;
        void.fmt(formatter)
    }
}
