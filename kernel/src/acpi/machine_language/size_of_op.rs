use super::Reader;

pub const SIZE_OF_OP: u8 = 0x87;

/// # SizeOfOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(Debug)]
pub struct SizeOfOp;

impl From<&[u8]> for SizeOfOp {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), SIZE_OF_OP);
        Self
    }
}

impl Reader<'_> for SizeOfOp {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        true
    }
}

