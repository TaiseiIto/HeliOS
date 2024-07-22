use {
    core::fmt,
    super::{
        ConstObj,
        ONE_OP,
        Reader,
        WORD_PREFIX,
        WordConst,
    },
};

/// # ComputationalData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub enum ComputationalData {
    ByteConst,
    WordConst(WordConst),
    DwordConst,
    QwordConst,
    String,
    ConstObj(ConstObj),
    RevisionOp,
    DefBuffer,
}

impl Reader<'_> for ComputationalData {
    pub fn length(&self) -> usize {
        match self {
            Self::ByteConst => unimplemented!(),
            Self::WordConst(word_const) => word_const.length(),
            Self::DwordConst => unimplemented!(),
            Self::QwordConst => unimplemented!(),
            Self::String => unimplemented!(),
            Self::ConstObj(const_obj) => const_obj.length(),
            Self::RevisionOp => unimplemented!(),
            Self::DefBuffer => unimplemented!(),
        }
    }
}

impl fmt::Debug for ComputationalData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ByteConst => write!(formatter, "ComputationalData::ByteConst"),
            Self::WordConst(word_const) => formatter
                .debug_tuple("ComputationalData")
                .field(word_const)
                .finish(),
            Self::DwordConst => write!(formatter, "ComputationalData::DwordConst"),
            Self::QwordConst => write!(formatter, "ComputationalData::QwordConst"),
            Self::String => write!(formatter, "ComputationalData::String"),
            Self::ConstObj(const_obj) => formatter
                .debug_tuple("ComputationalData")
                .field(const_obj)
                .finish(),
            Self::RevisionOp => write!(formatter, "ComputationalData::RevisionOp"),
            Self::DefBuffer => write!(formatter, "ComputationalData::DefBuffer"),
        }
    }
}

impl From<&[u8]> for ComputationalData {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            ONE_OP => Self::ConstObj(aml.into()),
            WORD_PREFIX => Self::WordConst(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

