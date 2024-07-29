use super::{
    Reader,
    TermArg,
};

/// # ArgObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(Debug)]
pub struct ArgObject(TermArg);

impl From<&[u8]> for ArgObject {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        Self(aml.into())
    }
}

impl Reader<'_> for ArgObject {
    fn length(&self) -> usize {
        self.0.length()
    }

    fn matches(aml: &[u8]) -> bool {
        TermArg::matches(aml)
    }
}

