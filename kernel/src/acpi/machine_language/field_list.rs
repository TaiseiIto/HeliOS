use {
    alloc::vec::Vec,
    super::{
        FieldElement,
        Reader,
    },
};

/// # FieldList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(Debug)]
pub struct FieldList(Vec<FieldElement>);

impl From<&[u8]> for FieldList {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let mut aml: &[u8] = aml;
        let mut field_list: Vec<FieldElement> = Vec::new();
        while !aml.is_empty() {
            let (field_element, remaining_aml): (FieldElement, &[u8]) = FieldElement::read(aml);
            aml = remaining_aml;
            field_list.push(field_element);
        }
        Self(field_list)
    }
}

impl Reader<'_> for FieldList {
    fn length(&self) -> usize {
        self.0
            .iter()
            .map(|field_element| field_element.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        if aml.first().is_some() {
            FieldElement::matches(aml)
        } else {
            true
        }
    }
}

