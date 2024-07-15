use super::ExtOpPrefix;

pub const OP_REGION_OP: u8 = 0x80;

/// # OpRegionOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(Debug)]
pub struct OpRegionOp {
    ext_op_prefix: ExtOpPrefix,
}

impl OpRegionOp {
    pub fn length(&self) -> usize {
        self.ext_op_prefix.length() + 1
    }
}

impl From<&[u8]> for OpRegionOp {
    fn from(aml: &[u8]) -> Self {
        let ext_op_prefix: ExtOpPrefix = aml.into();
        let aml: &[u8] = &aml[ext_op_prefix.length()..];
        assert_eq!(*aml.first().unwrap(), OP_REGION_OP);
        Self {
            ext_op_prefix,
        }
    }
}

