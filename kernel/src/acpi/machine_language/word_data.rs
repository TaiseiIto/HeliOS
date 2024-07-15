use core::slice;

/// # WordData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(Debug)]
pub struct WordData(u16);

impl WordData {
    pub fn length(&self) -> usize {
        2
    }
}

impl From<&[u8]> for WordData {
    fn from(aml: &[u8]) -> Self {
        let mut aml_iterator: slice::Iter<u8> = aml.iter();
        let low: u8 = *aml_iterator.next().unwrap();
        let high: u8 = *aml_iterator.next().unwrap();
        let word_data: u16 = (low as u16) + ((high as u16) << u8::BITS);
        Self(word_data)
    }
}

