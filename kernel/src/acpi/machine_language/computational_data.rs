use {
    core::fmt,
    super::{
        ByteConst,
        ConstObj,
        DWordConst,
        Reader,
        String,
        WordConst,
    },
};

/// # ComputationalData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub enum ComputationalData {
    ByteConst(ByteConst),
    ConstObj(ConstObj),
    DWordConst(DWordConst),
    String(String),
    WordConst(WordConst),
}

impl fmt::Debug for ComputationalData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("ComputationalData");
        match self {
            Self::ByteConst(byte_const) => debug_tuple.field(byte_const),
            Self::ConstObj(const_obj) => debug_tuple.field(const_obj),
            Self::DWordConst(d_word_const) => debug_tuple.field(d_word_const),
            Self::String(string) => debug_tuple.field(string),
            Self::WordConst(word_const) => debug_tuple.field(word_const),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for ComputationalData {
    fn from(aml: &[u8]) -> Self {
        if ByteConst::matches(aml) {
            Self::ByteConst(aml.into())
        } else if ConstObj::matches(aml) {
            Self::ConstObj(aml.into())
        } else if DWordConst::matches(aml) {
            Self::DWordConst(aml.into())
        } else if String::matches(aml) {
            Self::String(aml.into())
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
            Self::ByteConst(byte_const) => byte_const.length(),
            Self::ConstObj(const_obj) => const_obj.length(),
            Self::DWordConst(d_word_const) => d_word_const.length(),
            Self::String(string) => string.length(),
            Self::WordConst(word_const) => word_const.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        ByteConst::matches(aml)
        || ConstObj::matches(aml)
        || DWordConst::matches(aml)
        || String::matches(aml)
        || WordConst::matches(aml)
    }
}

