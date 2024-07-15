use {
    core::fmt,
    super::{
        WordData,
        WordPrefix,
    },
};

/// # WordConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub struct WordConst {
    word_prefix: WordPrefix,
    word_data: WordData,
}

impl WordConst {
    pub fn length(&self) -> usize {
        let Self {
            word_prefix,
            word_data,
        } = self;
        word_prefix.length() + word_data.length()
    }
}

impl fmt::Debug for WordConst {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self{
            word_prefix,
            word_data,
        } = self;
        formatter
            .debug_tuple("WordConst")
            .field(word_prefix)
            .field(word_data)
            .finish()
    }
}

impl From<&[u8]> for WordConst {
    fn from(aml: &[u8]) -> Self {
        let word_prefix: WordPrefix = aml.into();
        let aml: &[u8] = &aml[word_prefix.length()..];
        let word_data: WordData = aml.into();
        Self {
            word_prefix,
            word_data,
        }
    }
}

