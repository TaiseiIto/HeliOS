/// # FieldElement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(Debug)]
pub struct FieldElement;

impl FieldElement {
    pub fn length(&self) -> usize {
        unimplemented!()
    }
}

impl From<&[u8]> for FieldElement {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

