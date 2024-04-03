/// # Extended I/O Programmable Interrupt Controller (EIO PIC) Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.23 Extended I/O Programmable Interrupt Controller (EIO PIC) Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    version: u8,
    cascade_vector: u8,
    node: u8,
    node_map: u64,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

