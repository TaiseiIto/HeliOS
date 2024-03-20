use super::interrupt_source_override;

/// # Local APIC NMI Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.7 Local APIC NMI Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    acpi_processor_uid: u8,
    flags: interrupt_source_override::Flags,
    local_apic_lint: u8,
}

