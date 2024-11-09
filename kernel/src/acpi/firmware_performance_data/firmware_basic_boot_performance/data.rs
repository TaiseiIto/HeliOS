/// # Firmware Basic Boot Performance Data Record
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.24.7 Firmware Basic Boot Performance Data Record
#[derive(Debug)]
#[repr(packed)]
pub struct Record {
    #[allow(dead_code)]
    record_type: u16,
    length: u8,
    #[allow(dead_code)]
    revision: u8,
    __: u32,
    #[allow(dead_code)]
    reset_end: u64,
    #[allow(dead_code)]
    os_loader_load_image_start: u64,
    #[allow(dead_code)]
    os_loader_start_image_start: u64,
    #[allow(dead_code)]
    exit_boot_services_entry: u64,
    #[allow(dead_code)]
    exit_boot_services_exit: u64,
}

impl Record {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

