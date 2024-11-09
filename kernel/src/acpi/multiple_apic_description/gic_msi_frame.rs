use bitfield_struct::bitfield;

/// # GIC MSI Frame Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.16 GIC MSI Frame Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    __: u16,
    #[allow(dead_code)]
    gic_msi_frame_id: u32,
    #[allow(dead_code)]
    physical_base_address: u64,
    #[allow(dead_code)]
    flags: Flags,
    #[allow(dead_code)]
    spi_count: u16,
    #[allow(dead_code)]
    spi_base: u16,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

/// # GIC MSI Frame Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.16 Table 5.40 GIC MSI Frame Flags
#[bitfield(u32)]
pub struct Flags {
    spi_count_base_select: bool,
    #[bits(31)]
    __: u32,
}

