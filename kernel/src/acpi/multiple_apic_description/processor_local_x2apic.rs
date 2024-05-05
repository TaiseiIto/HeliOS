use super::processor_local_apic;

/// # Processor Local x2APIC Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.12 Processor Local x2APIC Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    reserved0: u16,
    #[allow(dead_code)]
    x2apic_id: u32,
    #[allow(dead_code)]
    flags: processor_local_apic::Flags,
    #[allow(dead_code)]
    acpi_processor_uid: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

