use {
    core::fmt,
    super::{
        ExtOpPrefix,
        Reader,
    },
};

pub const OP_REGION_OP: u8 = 0x80;

/// # OpRegionOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct OpRegionOp {
    ext_op_prefix: ExtOpPrefix,
}

impl fmt::Debug for OpRegionOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ext_op_prefix,
        } = self;
        formatter
            .debug_tuple("OpRegionOp")
            .field(ext_op_prefix)
            .finish()
    }
}

impl From<&[u8]> for OpRegionOp {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
        Self {
            ext_op_prefix,
        }
    }
}

impl Reader<'_> for OpRegionOp {
    fn length(&self) -> usize {
        self.ext_op_prefix.length() + 1
    }

    fn matches(aml: &[u8]) -> bool {
        if ExtOpPrefix::matches(aml) {
            let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
            aml
                .first()
                .is_some_and(|head| *head == OP_REGION_OP)
        } else {
            false
        }
    }
}

