use super::interrupt_source_override;

/// # Local x2APIC NMI Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.13 Local x2APIC NMI Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    struct_type: u8,
    length: u8,
    flags: interrupt_source_override::Flags,
    acpi_processor_uid: u32,
    local_x2apic_lint: u8,
    reserved0: [u8; 3],
}
