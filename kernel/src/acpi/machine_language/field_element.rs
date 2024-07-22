use super::{
    NamedField,
    Reader,
};

/// # FieldElement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(Debug)]
pub enum FieldElement {
    NamedField(NamedField),
}

impl From<&[u8]> for FieldElement {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() as char {
            'A'..='Z' | '_' => Self::NamedField(aml.into()),
            _ => match *aml.first().unwrap() {
                unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
            },
        }
    }
}

impl Reader<'_> for FieldElement {
    fn length(&self) -> usize {
        match self {
            Self::NamedField(named_field) => named_field.length(),
        }
    }
}

