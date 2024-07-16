pub const ONE_OP: u8 = 0x01;

/// # OneOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(Debug)]
pub struct OneOp;

impl OneOp {
    pub fn length(&self) -> usize {
        1
    }
}

impl From<&[u8]> for OneOp {
    fn from(aml: &[u8]) -> Self {
        assert_eq!(*aml.first().unwrap(), ONE_OP);
        Self
    }
}

