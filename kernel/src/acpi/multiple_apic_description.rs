use super::system_description;

/// # MADT
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12 Multiple APIC Description Table (MADT)
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    local_interrupt_controller_address: u32,
    flags: u32,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

