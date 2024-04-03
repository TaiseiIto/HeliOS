use bitfield_struct::bitfield;

/// # GICC Affinity Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.4 GICC Affinity Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    proximity_domain: u32,
    acpi_processor_id: u32,
    flags: Flags,
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
    #[bits(31, access = RO)]
    reserved0: u32,
}

