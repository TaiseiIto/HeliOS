/// # GIC Interrupt Translation Service (ITS) Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.18 GIC Interrupt Translation Service (ITS) Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    _0: u16,
    #[allow(dead_code)]
    gic_its_id: u32,
    #[allow(dead_code)]
    physical_base_address: u64,
    #[allow(dead_code)]
    _1: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

