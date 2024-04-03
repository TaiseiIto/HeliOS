use super::system_description;

/// # Boot Graphics Resource Table (BGRT)
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.23 Boot Graphics Resource Table (BGRT)
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    version: u16,
    status: u8,
    image_type: u8,
    image_address: u64,
    image_offset_x: u32,
    image_offset_y: u32,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

