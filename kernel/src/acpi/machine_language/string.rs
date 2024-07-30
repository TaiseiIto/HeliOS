use {
    core::fmt,
    super::{
        Reader,
        StringPrefix,
    },
};

/// # String
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub struct String {
    string_prefix: StringPrefix,
}

impl fmt::Debug for String {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            string_prefix,
        } = self;
        formatter
            .debug_tuple("String")
            .field(string_prefix)
            .finish()
    }
}

impl From<&[u8]> for String {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (string_prefix, aml): (StringPrefix, &[u8]) = StringPrefix::read(aml);
        unimplemented!()
    }
}

impl Reader<'_> for String {
    fn length(&self) -> usize {
        let Self {
            string_prefix,
        } = self;
        string_prefix.length()
    }

    fn matches(aml: &[u8]) -> bool {
        StringPrefix::matches(aml)
    }
}
