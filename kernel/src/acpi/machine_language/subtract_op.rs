use super::Reader;

pub const SUBTRACT_OP: u8 = 0x74;

/// # SubtractOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(Debug)]
pub struct SubtractOp;

impl From<&[u8]> for SubtractOp {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), SUBTRACT_OP);
        Self
    }
}

impl Reader<'_> for SubtractOp {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        true
    }
}

