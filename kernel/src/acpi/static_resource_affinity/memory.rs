use bitfield_struct::bitfield;

/// # Memory Affinity Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.2 Memory Affinity Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length0: u8,
    proximity_domain: u32,
    reserved0: u16,
    base_address: u64,
    length1: u64,
    reserved: u32,
    flags: Flags,
    reserved1: u64,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length0 as usize
    }
}

/// # Memory Affinity Structure Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.2 Processor Local APIC/SAPIC Affinity Structure
#[bitfield(u32)]
struct Flags {
    enabled: bool,
    hot_pluggable: bool,
    non_volatile: bool,
    #[bits(29, access = RO)]
    reserved0: u32,
}

