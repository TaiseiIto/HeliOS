use bitfield_struct::bitfield;

/// # Core Programmable Interrupt Controller (CORE PIC) Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.20 Core Programmable Interrupt Controller (CORE PIC) Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    version: u8,
    acpi_processor_id: u32,
    physical_processor_id: u32,
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
    #[bits(31, access = RO)]
    reserved: u32,
}

