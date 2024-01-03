//! # UEFI
//! ## References
//! * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf)

use core::fmt;

/// # A collection of related interfaces. Type VOID *.
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 2.3.1 Data Types
pub struct Handle<'a>(&'a ());

impl fmt::Debug for Handle<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#x?}", self.0 as *const () as usize)
    }
}

/// # Status codeo
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 2.3.1 Data Types
pub struct Status(usize);

/// # The operation was aborted.
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) Appendix D - Status Codes, Table D-3 EFI_STATUS Error Codes (High Bit Set)
pub const ABORTED: Status = Status(Status::ERROR + 21);

impl Status {
    /// # Error bit
    /// ## References
    /// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) Appendix D - Status Codes, Table D-1 EFI_STATUS Code Ranges
    const ERROR: usize = 1 << (usize::BITS - 1);
}

