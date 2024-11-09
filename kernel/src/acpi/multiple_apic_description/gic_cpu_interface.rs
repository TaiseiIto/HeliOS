use bitfield_struct::bitfield;

/// # GIC CPU Interface (GICC) Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.14 GIC CPU Interface (GICC) Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    reserved0: u16,
    #[allow(dead_code)]
    cpu_interface_number: u32,
    #[allow(dead_code)]
    acpi_processor_uid: u32,
    #[allow(dead_code)]
    flags: Flags,
    #[allow(dead_code)]
    parking_protocol_version: u32,
    #[allow(dead_code)]
    performance_interrupt_gsiv: u32,
    #[allow(dead_code)]
    parked_address: u64,
    #[allow(dead_code)]
    physical_base_address: u64,
    #[allow(dead_code)]
    gicv: u64,
    #[allow(dead_code)]
    gich: u64,
    #[allow(dead_code)]
    vgic_maintenance_interrupt: u32,
    #[allow(dead_code)]
    gicr_base_address: u64,
    #[allow(dead_code)]
    mpidr: u64,
    #[allow(dead_code)]
    processor_power_efficiency_class: u8,
    #[allow(dead_code)]
    reserved1: u8,
    #[allow(dead_code)]
    spe_overflow_interrupt: u16,
    #[allow(dead_code)]
    trbe_interrupt: u16,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
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
    #[bits(28)]
    __: u32,
}

