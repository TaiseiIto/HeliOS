use {
    core::fmt,
    super::{
        ByteData,
        Reader,
    },
};

/// # WordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub struct WordData {
    low: ByteData,
    high: ByteData,
}

impl fmt::Debug for WordData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            low,
            high,
        } = self;
        formatter
            .debug_tuple("WordData")
            .field(low)
            .field(high)
            .finish()
    }
}

impl From<&[u8]> for WordData {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (low, aml): (ByteData, &[u8]) = ByteData::read(aml);
        let (high, _aml): (ByteData, &[u8]) = ByteData::read(aml);
        Self {
            low,
            high,
        }
    }
}

impl Reader<'_> for WordData {
    fn length(&self) -> usize {
        let Self {
            low,
            high,
        } = self;
        low.length() + high.length()
    }

    fn matches(aml: &[u8]) -> bool {
        ByteData::matches(aml)
    }
}

