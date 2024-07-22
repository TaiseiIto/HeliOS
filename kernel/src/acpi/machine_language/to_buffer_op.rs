use super::Reader;

pub const TO_BUFFER_OP: u8 = 0x96;

/// # ToBufferOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(Debug)]
pub struct ToBufferOp;

impl From<&[u8]> for ToBufferOp {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), TO_BUFFER_OP);
        Self
    }
}

impl Reader<'_> for ToBufferOp {
    fn length(&self) -> usize {
        1
    }
}

