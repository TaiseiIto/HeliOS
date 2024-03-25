/// # Multiprocessor Wakeup Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.19 Multiprocessor Wakeup Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    mail_box_version: u16,
    reserved0: u32,
    mail_box_address: u64,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}
