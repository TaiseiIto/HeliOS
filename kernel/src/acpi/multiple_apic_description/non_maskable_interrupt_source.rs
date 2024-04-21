use super::interrupt_source_override;

/// # Non-Maskable Interrupt (NMI) Source Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.6 Non-Maskable Interrupt (NMI) Source Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    flags: interrupt_source_override::Flags,
    #[allow(dead_code)]
    global_system_interrupt: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}


