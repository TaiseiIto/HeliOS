use bitfield_struct::bitfield;

/// # Processor Local APIC Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.2 Processor Local APIC Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    acpi_processor_uid: u8,
    apic_id: u8,
    flags: Flags,
}

#[bitfield(u32)]
pub struct Flags {
    enabled: bool,
    online_capable: bool,
    #[bits(30, access = RO)]
    reserved0: u32,
}

