use {
    core::fmt,
    super::{
        ByteData,
        BytePrefix,
        Reader,
    },
};

/// # ByteConst
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub struct ByteConst {
    byte_prefix: BytePrefix,
    byte_data: ByteData,
}

impl fmt::Debug for ByteConst {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            byte_prefix,
            byte_data,
        } = self;
        formatter
            .debug_tuple("ByteConst")
            .field(byte_prefix)
            .field(byte_data)
            .finish()
    }
}

impl From<&[u8]> for ByteConst {
    fn from(aml: &[u8]) -> Self {
        let (byte_prefix, aml): (BytePrefix, &[u8]) = BytePrefix::read(aml);
        let (byte_data, _aml): (ByteData, &[u8]) = ByteData::read(aml);
        Self {
            byte_prefix,
            byte_data,
        }
    }
}

impl Reader<'_> for ByteConst {
    fn length(&self) -> usize {
        let Self {
            byte_prefix,
            byte_data,
        } = self;
        byte_prefix.length() + byte_data.length()
    }

    fn matches(aml: &[u8]) -> bool {
        BytePrefix::matches(aml)
    }
}

