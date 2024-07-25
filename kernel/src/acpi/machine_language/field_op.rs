use {
    core::fmt,
    super::{
        ExtOpPrefix,
        Reader,
    },
};

const FIELD_OP: u8 = 0x81;

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
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (ext_op_prefix, _aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
        Self {
            ext_op_prefix,
        }
    }
}

impl Reader<'_> for FieldOp {
    fn length(&self) -> usize {
        self.ext_op_prefix.length() + 1
    }

    fn matches(aml: &[u8]) -> bool {
        ExtOpPrefix::matches(aml) && {
            let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
            aml
                .first()
                .is_some_and(|head| *head == FIELD_OP)
        }
    }
}

