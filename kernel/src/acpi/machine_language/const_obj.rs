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
    ZeroOp,
    OneOp(OneOp),
    OnesOp,
}

impl fmt::Debug for ConstObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroOp => write!(formatter, "ConstObj::ZeroOp"),
            Self::OneOp(one_op) => formatter
                .debug_tuple("ConstObj")
                .field(one_op)
                .finish(),
            Self::OnesOp => write!(formatter, "ConstObj::OnesOp"),
        }
    }
}

impl From<&[u8]> for ConstObj {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            ONE_OP => Self::OneOp(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

impl Reader<'_> for ConstObj {
    fn length(&self) -> usize {
        match self {
            Self::ZeroOp => unimplemented!(),
            Self::OneOp(one_op) => one_op.length(),
            Self::OnesOp => unimplemented!(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        true
    }
}

