/// # Legacy IO Programmable Interrupt Controller (LIO PIC) Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.21 Legacy IO Programmable Interrupt Controller (LIO PIC) Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    version: u8,
    #[allow(dead_code)]
    base_address: u64,
    #[allow(dead_code)]
    size: u16,
    #[allow(dead_code)]
    cascade_vector: u16,
    #[allow(dead_code)]
    cascade_vector_mapping: u64,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}
