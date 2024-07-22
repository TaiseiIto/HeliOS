use super::{
    Reader,
    TermArg,
};

/// # Operand
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(Debug)]
pub struct Operand(TermArg);

impl From<&[u8]> for Operand {
    fn from(aml: &[u8]) -> Self {
        Self(aml.into())
    }
}

impl Reader<'_> for Operand {
    fn length(&self) -> usize {
        self.0.length()
    }
}

