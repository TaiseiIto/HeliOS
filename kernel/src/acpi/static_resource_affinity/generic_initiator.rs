use {
    bitfield_struct::bitfield,
    super::generic_port,
};

/// # Generic Initiator Affinity Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.6 Generic Initiator Affinity Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u8,
    length: u8,
    reserved0: u8,
    device_handle_type: u8,
    proximity_domain: u32,
    device_handle: u128,
    flags: generic_port::Flags,
    reserved1: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

