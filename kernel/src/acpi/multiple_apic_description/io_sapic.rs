/// # I/O SAPIC Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.9 I/O SAPIC Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    io_apic_id: u8,
    __: u8,
    #[allow(dead_code)]
    global_system_interrupt_base: u32,
    #[allow(dead_code)]
    io_sapic_address: u64,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

