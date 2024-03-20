use bitfield_struct::bitfield;

/// # Interrupt Source Override Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.5 Interrupt Source Override Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    bus: u8,
    source: u8,
    global_system_interrupt: u32,
    flags: Flags,
}

#[bitfield(u16)]
struct Flags {
    #[bits(2)]
    polarity: u8,
    #[bits(2)]
    trigger_mode: u8,
    #[bits(12, access = RO)]
    reserved0: u16,
}

