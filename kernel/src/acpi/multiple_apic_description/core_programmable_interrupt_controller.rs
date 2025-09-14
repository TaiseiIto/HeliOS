use bitfield_struct::bitfield;

/// # Core Programmable Interrupt Controller (CORE PIC) Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.20 Core Programmable Interrupt Controller (CORE PIC) Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    version: u8,
    #[allow(dead_code)]
    acpi_processor_id: u32,
    #[allow(dead_code)]
    physical_processor_id: u32,
    #[allow(dead_code)]
    flags: Flags,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

/// # CORE PIC Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.20 Table 5.46 CORE PIC Flags
#[bitfield(u32)]
pub struct Flags {
    enabled: bool,
    #[bits(31)]
    __: u32,
}
