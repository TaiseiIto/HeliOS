use super::super::super::record;

/// # Firmware Basic Boot Performance Table Pointer Record
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.24.4 Firmware Basic Boot Performance Table Pointer Record
#[derive(Debug)]
#[repr(packed)]
pub struct Record {
    header: record::Header,
    reserved0: u32,
    fbpt: u64,
}

impl Record {
    pub fn length(&self) -> usize {
        self.header.length()
    }
}

