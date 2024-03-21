use bitfield_struct::bitfield;

/// # GIC CPU Interface (GICC) Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.14 GIC CPU Interface (GICC) Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    reserved0: u16,
    cpu_interface_number: u32,
    acpi_processor_uid: u32,
    flags: Flags,
    parking_protocol_version: u32,
    performance_interrupt_gsiv: u32,
    parked_address: u64,
    physical_base_address: u64,
    gicv: u64,
    gich: u64,
    vgic_maintenance_interrupt: u32,
    gicr_base_address: u64,
    mpidr: u64,
    processor_power_efficiency_class: u8,
    reserved1: u8,
    spe_overflow_interrupt: u16,
    trbe_interrupt: u16,
}

/// # GICC CPU Interface Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.14 Table 5.37 GICC CPU Interface Flags
#[bitfield(u32)]
pub struct Flags {
    enabled: bool,
    performance_interrupt_mode: bool,
    vgic_maintenance_interrupt_mode_flags: bool,
    online_capable: bool,
    #[bits(28, access = RO)]
    reserved2: u32,
}

