use bitfield_struct::bitfield;

/// # MethodFlags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[bitfield(u8)]
pub struct MethodFlags {
    #[bits(3)]
    arg_count: u8,
    serialize: bool,
    #[bits(4)]
    sync_level: u8,
}

impl MethodFlags {
    pub fn length(&self) -> usize {
        1
    }
}

impl From<&[u8]> for MethodFlags {
    fn from(aml: &[u8]) -> Self {
        (*aml.first().unwrap()).into()
    }
}

