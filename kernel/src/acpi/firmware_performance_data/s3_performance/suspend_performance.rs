/// # Basic S3 Suspend Performance Record
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.24.8 Table 5.116 Basic S3 Suspend Performance Record
#[derive(Debug)]
#[repr(packed)]
pub struct Record {
    #[allow(dead_code)]
    record_type: u16,
    length: u8,
    #[allow(dead_code)]
    revision: u8,
    #[allow(dead_code)]
    suspend_start: u64,
    #[allow(dead_code)]
    suspend_end: u64,
}

impl Record {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}
