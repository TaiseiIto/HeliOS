use {
    core::fmt,
    super::{
        AcquireOpSuffix,
        ExtOpPrefix,
        Reader,
    },
};

/// # AcquireOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct AcquireOp {
    ext_op_prefix: ExtOpPrefix,
    acquire_op_suffix: AcquireOpSuffix,
}

impl fmt::Debug for AcquireOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ext_op_prefix,
            acquire_op_suffix,
        } = self;
        formatter
            .debug_tuple("AcquireOp")
            .field(ext_op_prefix)
            .field(acquire_op_suffix)
            .finish()
    }
}

impl From<&[u8]> for AcquireOp {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
        let (acquire_op_suffix, aml): (AcquireOpSuffix, &[u8]) = AcquireOpSuffix::read(aml);
        Self {
            ext_op_prefix,
            acquire_op_suffix,
        }
    }
}

impl Reader<'_> for AcquireOp {
    fn length(&self) -> usize {
        let Self {
            ext_op_prefix,
            acquire_op_suffix,
        } = self;
        ext_op_prefix.length() + acquire_op_suffix.length()
    }

    fn matches(aml: &[u8]) -> bool {
        if !ExtOpPrefix::matches(aml) {
            return false;
        }
        let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
        if !AcquireOpSuffix::matches(aml) {
            return false;
        }
        let (acquire_op_suffix, aml): (AcquireOpSuffix, &[u8]) = AcquireOpSuffix::read(aml);
        true
    }
}

