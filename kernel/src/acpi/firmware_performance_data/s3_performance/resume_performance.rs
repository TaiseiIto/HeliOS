/// # Basic S3 Resume Performance Record
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.24.8 Table 5.115 Basic S3 Resume Performance Record
#[derive(Debug)]
#[repr(packed)]
pub struct Record {
    #[allow(dead_code)]
    record_type: u16,
    length: u8,
    #[allow(dead_code)]
    revision: u8,
    #[allow(dead_code)]
    resume_count: u32,
    #[allow(dead_code)]
    full_resume: u64,
    #[allow(dead_code)]
    average_resume: u64,
}

impl Record {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}
