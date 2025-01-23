use bitfield_struct::bitfield;

/// # Generic Port Affinity Structure
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.7 Generic Port Affinity Structure
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
    flags: Flags,
    #[allow(dead_code)]
    _1: u32,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

/// # Generic Port Affinity Structure Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16.7 Generic Port Affinity Structure
#[bitfield(u32)]
pub struct Flags {
    enabled: bool,
    architectural_transactions: bool,
    #[bits(30)]
    __: u32,
}

