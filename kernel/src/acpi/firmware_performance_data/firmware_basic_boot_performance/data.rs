/// # Firmware Basic Boot Performance Data Record
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.24.7 Firmware Basic Boot Performance Data Record
#[derive(Debug)]
#[repr(packed)]
pub struct Record {
    record_type: u16,
    length: u8,
    revision: u8,
    reserved0: u32,
    reset_end: u64,
    os_loader_load_image_start: u64,
    os_loader_start_image_start: u64,
    exit_boot_services_entry: u64,
    exit_boot_services_exit: u64,
}

impl Record {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

