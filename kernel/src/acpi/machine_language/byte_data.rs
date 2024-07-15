/// # ByteData
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(Debug)]
pub struct ByteData(u8);

impl ByteData {
    pub fn length(&self) -> usize {
        1
    }

    pub fn pkg_length(&self) -> usize {
        self.0 as usize
    }
}

impl From<&[u8]> for ByteData {
    fn from(aml: &[u8]) -> Self {
        Self(*aml.first().unwrap())
    }
}
