use {
    core::fmt,
    super::{
        WordData,
        Reader,
    },
};

/// # DWordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub struct DWordData {
    low: WordData,
    high: WordData,
}

impl fmt::Debug for DWordData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            low,
            high,
        } = self;
        formatter
            .debug_tuple("DWordData")
            .field(low)
            .field(high)
            .finish()
    }
}

impl From<&[u8]> for DWordData {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (low, aml): (WordData, &[u8]) = WordData::read(aml);
        let (high, _aml): (WordData, &[u8]) = WordData::read(aml);
        Self {
            low,
            high,
        }
    }
}

impl Reader<'_> for DWordData {
    fn length(&self) -> usize {
        let Self {
            low,
            high,
        } = self;
        low.length() + high.length()
    }

    fn matches(aml: &[u8]) -> bool {
        WordData::matches(aml)
    }
}

