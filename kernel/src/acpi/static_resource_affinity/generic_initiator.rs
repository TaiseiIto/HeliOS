use super::generic_port;

/// # Generic Initiator Affinity Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.6 Generic Initiator Affinity Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    #[allow(dead_code)]
    structure_type: u8,
    length: u8,
    #[allow(dead_code)]
    _0: u8,
    #[allow(dead_code)]
    device_handle_type: u8,
    #[allow(dead_code)]
    proximity_domain: u32,
    #[allow(dead_code)]
    device_handle: u128,
    #[allow(dead_code)]
    flags: generic_port::Flags,
    #[allow(dead_code)]
    _1: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

