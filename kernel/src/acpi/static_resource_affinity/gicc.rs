use bitfield_struct::bitfield;

/// # GICC Affinity Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.4 GICC Affinity Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    proximity_domain: u32,
    #[allow(dead_code)]
    acpi_processor_id: u32,
    #[allow(dead_code)]
    flags: Flags,
    #[allow(dead_code)]
    clock_domain: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

/// # GICC Affinity Structure Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.4 GICC Affinity Structure
#[bitfield(u32)]
struct Flags {
    enabled: bool,
    #[bits(31)]
    __: u32,
}
