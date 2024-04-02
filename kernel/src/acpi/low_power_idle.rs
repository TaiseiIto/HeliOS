use super::system_description;

/// # Low Power Idle Table (LPIT)
/// ## References
/// * [Intel Low Power S0 Idle](https://uefi.org/sites/default/files/resources/Intel_ACPI_Low_Power_S0_Idle.pdf) 2.1 LPTI Table Main Structure
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

