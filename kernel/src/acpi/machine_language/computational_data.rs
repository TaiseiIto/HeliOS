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
    WordConst(WordConst),
    ConstObj(ConstObj),
}

impl fmt::Debug for ComputationalData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WordConst(word_const) => formatter
                .debug_tuple("ComputationalData")
                .field(word_const)
                .finish(),
            Self::ConstObj(const_obj) => formatter
                .debug_tuple("ComputationalData")
                .field(const_obj)
                .finish(),
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

impl Reader<'_> for ComputationalData {
    fn length(&self) -> usize {
        match self {
            Self::WordConst(word_const) => word_const.length(),
            Self::ConstObj(const_obj) => const_obj.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        WordConst::matches(aml)
        || ConstObj::matches(aml)
    }
}

