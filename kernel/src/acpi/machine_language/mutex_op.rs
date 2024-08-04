use {
    core::fmt,
    super::{
        ExtOpPrefix,
        MutexOpSuffix,
        Reader,
    },
};

/// # MutexOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct MutexOp {
    ext_op_prefix: ExtOpPrefix,
    mutex_op_suffix: MutexOpSuffix,
}

impl fmt::Debug for MutexOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ext_op_prefix,
            mutex_op_suffix,
        } = self;
        formatter
            .debug_tuple("MutexOp")
            .field(ext_op_prefix)
            .field(mutex_op_suffix)
            .finish()
    }
}

impl From<&[u8]> for MutexOp {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
        let (mutex_op_suffix, _aml): (MutexOpSuffix, &[u8]) = MutexOpSuffix::read(aml);
        Self {
            ext_op_prefix,
            mutex_op_suffix,
        }
    }
}

impl Reader<'_> for MutexOp {
    fn length(&self) -> usize {
        let Self {
            ext_op_prefix,
            mutex_op_suffix,
        } = self;
        ext_op_prefix.length() + mutex_op_suffix.length()
    }

    fn matches(aml: &[u8]) -> bool {
        ExtOpPrefix::matches(aml) && {
            let (_ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
            MutexOpSuffix::matches(aml)
        }
    }
}

