/// # GIC Interrupt Translation Service (ITS) Affinity Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.5 GIC Interrupt Translation Service (ITS) Affinity Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    proximity_domain: u32,
    #[allow(dead_code)]
    reserved0: u16,
    #[allow(dead_code)]
    its_id: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

