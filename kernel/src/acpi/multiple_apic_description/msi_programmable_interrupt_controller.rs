/// # MSI Programmable Interrupt Controller (MSI PIC) Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.24 MSI Programmable Interrupt Controller (MSI PIC) Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    version: u8,
    message_address: u64,
    start: u32,
    count: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

