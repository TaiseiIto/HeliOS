use {
    bitfield_struct::bitfield,
    super::Reader,
};

/// # FieldFlags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[bitfield(u8)]
pub struct FieldFlags {
    #[bits(4)]
    access_type: u8,
    lock_rule: bool,
    #[bits(2)]
    update_rule: u8,
    #[bits(access = RO)]
    reserved0: bool,
}

impl From<&[u8]> for FieldFlags {
    fn from(aml: &[u8]) -> Self {
        (*aml.first().unwrap()).into()
    }
}

impl Reader<'_> for FieldFlags {
    fn length(&self) -> usize {
        1
    }
}

