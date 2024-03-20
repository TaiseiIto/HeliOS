/// # I/O APIC Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12.3 I/O APIC Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    io_apic_id: u8,
    reserved0: u8,
    io_apic_address: u32,
    global_system_interrupt_base: u32,
}

