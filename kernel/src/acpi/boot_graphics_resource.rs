use super::system_description;

/// # Boot Graphics Resource Table (BGRT)
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.23 Boot Graphics Resource Table (BGRT)
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    #[allow(dead_code)]
    version: u16,
    #[allow(dead_code)]
    status: u8,
    #[allow(dead_code)]
    image_type: u8,
    #[allow(dead_code)]
    image_address: u64,
    #[allow(dead_code)]
    image_offset_x: u32,
    #[allow(dead_code)]
    image_offset_y: u32,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}
