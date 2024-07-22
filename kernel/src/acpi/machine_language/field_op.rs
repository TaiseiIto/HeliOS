use {
    core::fmt,
    super::{
        ExtOpPrefix,
        Reader,
    },
};

pub const FIELD_OP: u8 = 0x81;

/// # OpRegionOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct FieldOp {
    ext_op_prefix: ExtOpPrefix,
}

impl fmt::Debug for FieldOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ext_op_prefix,
        } = self;
        formatter
            .debug_tuple("FieldOp")
            .field(ext_op_prefix)
            .finish()
    }
}

impl From<&[u8]> for FieldOp {
    fn from(aml: &[u8]) -> Self {
        let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
        assert_eq!(*aml.first().unwrap(), FIELD_OP);
        Self {
            ext_op_prefix,
        }
    }
}

impl Reader<'_> for FieldOp {
    fn length(&self) -> usize {
        self.ext_op_prefix.length() + 1
    }
}
