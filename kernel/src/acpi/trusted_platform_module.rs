use super::system_description;

/// # Trusted Platform Module 2.0 Table
/// ## References
/// * [TCG ACPI Specification](https://trustedcomputinggroup.org/wp-content/uploads/TCG_ACPIGeneralSpec_v1p3_r8_pub.pdf) 8.3 ACPI Table for TPM 2.0
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    platform_class: u16,
    reserved0: u16,
    address_of_crb_control_area_or_fifo_base_address: u64,
    start_method: u32,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

