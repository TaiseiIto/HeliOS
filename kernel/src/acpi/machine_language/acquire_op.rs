use {
    core::fmt,
    super::{
        Reader,
        ExtOpPrefix,
    },
};

const ACQUIRE_OP: u8 = 0x23;

/// # AcquireOp
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct AcquireOp {
    ext_op_prefix: ExtOpPrefix,
}

impl fmt::Debug for AcquireOp {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            ext_op_prefix,
        } = self;
        formatter
            .debug_tuple("AcquireOp")
            .field(ext_op_prefix)
            .finish()
    }
}

impl From<&[u8]> for AcquireOp {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
        Self {
            ext_op_prefix,
        }
    }
}

impl Reader<'_> for AcquireOp {
    fn length(&self) -> usize {
        self.ext_op_prefix.length() + 1
    }

    fn matches(aml: &[u8]) -> bool {
        ExtOpPrefix::matches(aml) && {
            let (ext_op_prefix, aml): (ExtOpPrefix, &[u8]) = ExtOpPrefix::read(aml);
            aml
                .first()
                .is_some_and(|head| *head == ACQUIRE_OP)
        }
    }
}

