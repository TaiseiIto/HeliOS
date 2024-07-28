use super::Reader;

const L_EQUAL_OP: u8 = 0x93;

/// # LEqualOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(Debug)]
pub struct LEqualOp;

impl From<&[u8]> for LEqualOp {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        Self
    }
}

impl Reader<'_> for LEqualOp {
    fn length(&self) -> usize {
        1
    }

    fn matches(aml: &[u8]) -> bool {
        aml
        .first()
        .is_some_and(|head| *head == L_EQUAL_OP)
    }
}
