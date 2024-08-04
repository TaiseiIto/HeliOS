use {
    core::fmt,
    super::{
        ExtOpPrefix,
        OpRegionOpSuffix,
        Reader,
    },
};

/// # OpRegionOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct OpRegionOp {
    ext_op_prefix: ExtOpPrefix,
    op_region_op_suffix: OpRegionOpSuffix,
}

impl fmt::Debug for OpRegionOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ext_op_prefix,
            op_region_op_suffix,
        } = self;
        formatter
            .debug_tuple("OpRegionOp")
            .field(ext_op_prefix)
            .field(op_region_op_suffix)
            .finish()
    }
}

impl From<&[u8]> for OpRegionOp {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
        let (op_region_op_suffix, _aml): (OpRegionOpSuffix, &[u8]) = OpRegionOpSuffix::read(aml);
        Self {
            ext_op_prefix,
            op_region_op_suffix,
        }
    }
}

impl Reader<'_> for OpRegionOp {
    fn length(&self) -> usize {
        let Self {
            ext_op_prefix,
            op_region_op_suffix,
        } = self;
        ext_op_prefix.length() + op_region_op_suffix.length()
    }

    fn matches(aml: &[u8]) -> bool {
        ExtOpPrefix::matches(aml) && {
            let (_ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
            OpRegionOpSuffix::matches(aml)
        }
    }
}

