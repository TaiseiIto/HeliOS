use super::Reader;

pub const METHOD_OP: u8 = 0x14;

/// # MethodOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(Debug)]
pub struct MethodOp;

impl From<&[u8]> for MethodOp {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), METHOD_OP);
        Self
    }
}

impl Reader<'_> for MethodOp {
    fn length(&self) -> usize {
        1
    }
}

