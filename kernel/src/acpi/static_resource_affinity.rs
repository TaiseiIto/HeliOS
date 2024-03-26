use super::system_description;

/// # System Resource Affinity Table (SRAT)
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16 System Resource Affinity Table (SRAT)
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    reserved0: [u8; 12],
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

