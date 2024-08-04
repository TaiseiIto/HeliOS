use {
    core::fmt,
    super::{
        ExtOpPrefix,
        FieldOpSuffix,
        Reader,
    },
};

/// # FieldOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct FieldOp {
    ext_op_prefix: ExtOpPrefix,
    field_op_suffix: FieldOpSuffix,
}

impl fmt::Debug for FieldOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ext_op_prefix,
            field_op_suffix,
        } = self;
        formatter
            .debug_tuple("FieldOp")
            .field(ext_op_prefix)
            .field(field_op_suffix)
            .finish()
    }
}

impl From<&[u8]> for FieldOp {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
        let (field_op_suffix, _aml): (FieldOpSuffix, &[u8]) = FieldOpSuffix::read(aml);
        Self {
            ext_op_prefix,
            field_op_suffix,
        }
    }
}

impl Reader<'_> for FieldOp {
    fn length(&self) -> usize {
        let Self {
            ext_op_prefix,
            field_op_suffix,
        } = self;
        ext_op_prefix.length() + field_op_suffix.length()
    }

    fn matches(aml: &[u8]) -> bool {
        ExtOpPrefix::matches(aml) && {
            let (_ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
            FieldOpSuffix::matches(aml)
        }
    }
}

