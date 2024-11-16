use super::super::{
    super::record,
    Table,
};

/// # S3 Performance Table Pointer Record
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.24.5 S3 Performance Table Pointer Record
#[derive(Debug)]
#[repr(packed)]
pub struct Record<'a> {
    header: record::Header,
    __: u32,
    #[allow(dead_code)]
    s3pt: &'a Table,
}

impl Record<'_> {
    pub fn length(&self) -> usize {
        self.header.length()
    }
}

