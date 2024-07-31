use super::{
    Reader,
    SuperName,
};

/// # MutexObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(Debug)]
pub struct MutexObject(SuperName);

impl From<&[u8]> for MutexObject {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (super_name, aml): (SuperName, &[u8]) = SuperName::read(aml);
        Self(super_name)
    }
}

impl Reader<'_> for MutexObject {
    fn length(&self) -> usize {
        self.0.length()
    }

    fn matches(aml: &[u8]) -> bool {
        SuperName::matches(aml)
    }
}

