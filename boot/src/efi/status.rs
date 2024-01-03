/// # EFI_STATUS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 2.3.1 Data Types
pub struct Status(usize);

/// # EFI_ABORTED
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) Appendix D - Status Codes, Table D-3 EFI_STATUS Error Codes (High Bit Set)
pub const ABORTED: Status = Status(Status::ERROR + 21);

impl Status {
    /// # Error bit
    /// ## References
    /// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) Appendix D - Status Codes, Table D-1 EFI_STATUS Code Ranges
    const ERROR: usize = 1 << (usize::BITS - 1);
}

