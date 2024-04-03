use super::system_description;

/// # XSDT
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.8 Extended System Description Table (XSDT)
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

