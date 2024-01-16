/// # EFI_STATUS
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 2.3.1 Data Types
#[derive(Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Status(usize);

impl Status {
    /// # EFI_SUCCESS
    /// ## References
    /// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) Appendix D - Status Codes, Table D-2 EFI_STATUS Success Codes (High Bit Clear)
    pub const SUCCESS: Self = Status(0);
    /// # EFI_BUFFER_TOO_SMAPP
    /// ## References
    /// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) Appendix D - Status Codes, Table D-3 EFI_STATUS Error Codes (High Bit Set)
    pub const BUFFER_TOO_SMALL: Self = Status(Status::ERROR + 5);
    /// # EFI_ABORTED
    /// ## References
    /// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) Appendix D - Status Codes, Table D-3 EFI_STATUS Error Codes (High Bit Set)
    pub const ABORTED: Self = Status(Status::ERROR + 21);
    /// # Error bit
    /// ## References
    /// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) Appendix D - Status Codes, Table D-1 EFI_STATUS Code Ranges
    const ERROR: usize = 1 << (usize::BITS - 1);
}

impl From<Status> for Result<(), Status> {
    fn from(status: Status) -> Self {
        match status {
            Status::SUCCESS => Ok(()),
            _ => Err(status),
        }
    }
}

