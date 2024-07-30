use {
    core::fmt,
    super::{
        AsciiCharList,
        NullChar,
        Reader,
        StringPrefix,
    },
};

/// # String
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub struct String {
    string_prefix: StringPrefix,
    ascii_char_list: AsciiCharList,
    null_char: NullChar,
}

impl fmt::Debug for String {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            string_prefix,
            ascii_char_list,
            null_char,
        } = self;
        formatter
            .debug_tuple("String")
            .field(string_prefix)
            .field(ascii_char_list)
            .field(null_char)
            .finish()
    }
}

impl From<&[u8]> for String {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (string_prefix, aml): (StringPrefix, &[u8]) = StringPrefix::read(aml);
        let (ascii_char_list, aml): (AsciiCharList, &[u8]) = AsciiCharList::read(aml);
        let (null_char, _aml): (NullChar, &[u8]) = NullChar::read(aml);
        Self {
            string_prefix,
            ascii_char_list,
            null_char,
        }
    }
}

impl Reader<'_> for String {
    fn length(&self) -> usize {
        let Self {
            string_prefix,
            ascii_char_list,
            null_char,
        } = self;
        string_prefix.length()
        + ascii_char_list.length()
        + null_char.length()
    }

    fn matches(aml: &[u8]) -> bool {
        StringPrefix::matches(aml)
    }
}

