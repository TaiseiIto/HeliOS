use {
    core::fmt,
    super::{
        DWordData,
        DWordPrefix,
        Reader,
    },
};

/// # DWordConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub struct DWordConst {
    d_word_prefix: DWordPrefix,
    d_word_data: DWordData,
}

impl fmt::Debug for DWordConst {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            d_word_prefix,
            d_word_data,
        } = self;
        formatter
            .debug_tuple("DWordConst")
            .field(d_word_prefix)
            .field(d_word_data)
            .finish()
    }
}

impl From<&[u8]> for DWordConst {
    fn from(aml: &[u8]) -> Self {
        let (d_word_prefix, aml): (DWordPrefix, &[u8]) = DWordPrefix::read(aml);
        let (d_word_data, _aml): (DWordData, &[u8]) = DWordData::read(aml);
        Self {
            d_word_prefix,
            d_word_data,
        }
    }
}

impl Reader<'_> for DWordConst {
    fn length(&self) -> usize {
        let Self {
            d_word_prefix,
            d_word_data,
        } = self;
        d_word_prefix.length() + d_word_data.length()
    }

    fn matches(aml: &[u8]) -> bool {
        DWordPrefix::matches(aml)
    }
}

