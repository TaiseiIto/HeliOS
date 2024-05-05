use super::interrupt_source_override;

/// # Local x2APIC NMI Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.13 Local x2APIC NMI Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    struct_type: u8,
    length: u8,
    #[allow(dead_code)]
    flags: interrupt_source_override::Flags,
    #[allow(dead_code)]
    acpi_processor_uid: u32,
    #[allow(dead_code)]
    local_x2apic_lint: u8,
    #[allow(dead_code)]
    reserved0: [u8; 3],
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

