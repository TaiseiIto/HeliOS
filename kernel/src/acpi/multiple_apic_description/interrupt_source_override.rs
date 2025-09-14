use bitfield_struct::bitfield;

/// # Interrupt Source Override Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.5 Interrupt Source Override Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    bus: u8,
    #[allow(dead_code)]
    source: u8,
    #[allow(dead_code)]
    global_system_interrupt: u32,
    #[allow(dead_code)]
    flags: Flags,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

/// # MPS INTI Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.5 Table 5.26 MPS INTI Flags
#[bitfield(u16)]
pub struct Flags {
    #[bits(2)]
    polarity: u8,
    #[bits(2)]
    trigger_mode: u8,
    #[bits(12)]
    __: u16,
}
