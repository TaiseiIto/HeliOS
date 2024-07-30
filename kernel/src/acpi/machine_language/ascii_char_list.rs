use {
    alloc::vec::Vec,
    super::{
        AsciiChar,
        Reader,
    },
};

/// # AsciiCharList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(Debug)]
pub struct AsciiCharList(Vec<AsciiChar>);

impl From<&[u8]> for AsciiCharList {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let mut aml: &[u8] = aml;
        let mut ascii_char_list: Vec<AsciiChar> = Vec::new();
        while AsciiChar::matches(aml) {
            let (ascii_char, remaining_aml): (AsciiChar, &[u8]) = AsciiChar::read(aml);
            aml = remaining_aml;
            ascii_char_list.push(ascii_char);
        }
        Self(ascii_char_list)
    }
}

impl Reader<'_> for AsciiCharList {
    fn length(&self) -> usize {
        self.0
            .iter()
            .map(|ascii_char| ascii_char.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        if aml.is_empty() {
            true
        } else {
            AsciiChar::matches(aml)
        }
    }
}

