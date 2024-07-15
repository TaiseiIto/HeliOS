pub const SCOPE_OP: u8 = 0x10;

/// # ScopeOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(Debug)]
pub struct ScopeOp;

impl ScopeOp {
    pub fn length(&self) -> usize {
        1
    }
}

impl From<&[u8]> for ScopeOp {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), SCOPE_OP);
        Self
    }
}

