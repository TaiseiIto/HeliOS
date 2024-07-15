use {
    core::fmt,
    super::WordPrefix,
};

/// # WordConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub struct WordConst {
    word_prefix: WordPrefix,
}

impl fmt::Debug for WordConst {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self{
            word_prefix,
        } = self;
        formatter
            .debug_tuple("WordConst")
            .field(word_prefix)
            .finish()
    }
}

impl From<&[u8]> for WordConst {
    fn from(aml: &[u8]) -> Self {
        let word_prefix: WordPrefix = aml.into();
        let aml: &[u8] = &aml[word_prefix.length()..];
        unimplemented!()
    }
}

