use {
    alloc::vec::Vec,
    super::{
        ByteData,
        Reader,
    },
};

/// # ByteList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(Debug)]
pub struct ByteList(Vec<ByteData>);

impl From<&[u8]> for ByteList {
    fn from(aml: &[u8]) -> Self {
        let mut byte_list: Vec<ByteData> = Vec::new();
        let mut aml: &[u8] = aml;
        while !aml.is_empty() {
            let (byte_data, remaining_aml): (ByteData, &[u8]) = ByteData::read(aml);
            aml = remaining_aml;
            byte_list.push(byte_data);
        }
        Self(byte_list)
    }
}

impl Reader<'_> for ByteList {
    fn length(&self) -> usize {
        self.0
            .iter()
            .map(|byte_data| byte_data.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        true
    }
}

