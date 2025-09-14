use super::processor_local_apic_sapic;

/// # Processor Local x2APIC Affinity Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.3 Processor Local x2APIC Affinity Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    _0: u16,
    #[allow(dead_code)]
    proximity_domain: u32,
    #[allow(dead_code)]
    x2apic_id: u32,
    #[allow(dead_code)]
    flags: processor_local_apic_sapic::Flags,
    #[allow(dead_code)]
    clock_domain: u32,
    #[allow(dead_code)]
    _1: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}
