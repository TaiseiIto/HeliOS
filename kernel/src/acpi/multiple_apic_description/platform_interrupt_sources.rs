use {
    bitfield_struct::bitfield,
    super::interrupt_source_override,
};

/// # Platform Interrupt Sources
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.11 Platform Interrupt Sources
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    flags: interrupt_source_override::Flags,
    interrupt_type: u8,
    processor_id: u8,
    processor_eid: u8,
    io_sapic_vector: u8,
    global_system_interrupt: u32,
    platform_interrupt_source_flags: Flags,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

/// # Platform Interrupt Source Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.11 Table 5.33 Platform Interrupt Source Flags
#[bitfield(u32)]
pub struct Flags {
    cpei_processor_override: bool,
    #[bits(31, access = RO)]
    reserved0: u32,
}

