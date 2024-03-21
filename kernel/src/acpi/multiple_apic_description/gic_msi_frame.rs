use bitfield_struct::bitfield;

/// # GIC MSI Frame Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.16 GIC MSI Frame Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    reserved0: u16,
    gic_msi_frame_id: u32,
    physical_base_address: u64,
    flags: Flags,
    spi_count: u16,
    spi_base: u16,
}

/// # GIC MSI Frame Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.16 Table 5.40 GIC MSI Frame Flags
#[bitfield(u32)]
pub struct Flags {
    spi_count_base_select: bool,
    #[bits(31, access = RO)]
    reserved0: u32,
}

