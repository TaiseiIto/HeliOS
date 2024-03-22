/// # HyperTransport Programmable Interrupt Controller (HT PIC) Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.22 HyperTransport Programmable Interrupt Controller (HT PIC) Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    version: u8,
    base_address: u64,
    size: u16,
    cascade_vector: u64,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

