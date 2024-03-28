use {
    bitfield_struct::bitfield,
    super::processor_local_apic_sapic,
};

/// # Processor Local x2APIC Affinity Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.3 Processor Local x2APIC Affinity Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    reserved0: u16,
    proximity_domain: u32,
    x2apic_id: u32,
    flags: processor_local_apic_sapic::Flags,
    clock_domain: u32,
    reserved1: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

