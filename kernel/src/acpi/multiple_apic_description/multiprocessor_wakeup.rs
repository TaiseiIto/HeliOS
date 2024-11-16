/// # Multiprocessor Wakeup Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.19 Multiprocessor Wakeup Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    mail_box_version: u16,
    __: u32,
    #[allow(dead_code)]
    mail_box_address: u64,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

