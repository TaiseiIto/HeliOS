use {
    core::fmt,
    super::{
        ConstObj,
        Reader,
        WordConst,
    },
};

/// # ComputationalData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub enum ComputationalData {
    ConstObj(ConstObj),
    WordConst(WordConst),
}

impl fmt::Debug for ComputationalData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("ComputationalData");
        match self {
            Self::WordConst(word_const) => debug_tuple.field(word_const),
            Self::ConstObj(const_obj) => debug_tuple.field(const_obj),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for ComputationalData {
    fn from(aml: &[u8]) -> Self {
        if ConstObj::matches(aml) {
            Self::ConstObj(aml.into())
        } else if WordConst::matches(aml) {
            Self::WordConst(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
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

