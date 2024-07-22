use super::Reader;

pub const TO_HEX_STRING_OP: u8 = 0x98;

/// # ToHexStringOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(Debug)]
pub struct ToHexStringOp;

impl From<&[u8]> for ToHexStringOp {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), TO_HEX_STRING_OP);
        Self
    }
}

impl Reader<'_> for ToHexStringOp {
    fn length(&self) -> usize {
        1
    }
}

