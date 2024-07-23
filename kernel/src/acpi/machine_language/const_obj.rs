use {
    core::fmt,
    super::{
        OneOp,
        Reader,
    },
};

/// # ConstObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub enum ConstObj {
    OneOp(OneOp),
}

impl fmt::Debug for ConstObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneOp(one_op) => formatter
                .debug_tuple("ConstObj")
                .field(one_op)
                .finish(),
        }
    }
}

impl From<&[u8]> for ConstObj {
    fn from(aml: &[u8]) -> Self {
        if OneOp::matches(aml) {
            Self::OneOp(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for ConstObj {
    fn length(&self) -> usize {
        match self {
            Self::OneOp(one_op) => one_op.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        OneOp::matches(aml)
    }
}

