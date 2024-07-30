use {
    alloc::vec::Vec,
    super::{
        PackageElement,
        Reader,
    },
};

/// # PackageElementList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(Debug)]
pub struct PackageElementList(Vec<PackageElement>);

impl From<&[u8]> for PackageElementList {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let mut aml: &[u8] = aml;
        let mut package_element_list: Vec<PackageElement> = Vec::new();
        while !aml.is_empty() {
            let (package_element, remaining_aml): (PackageElement, &[u8]) = PackageElement::read(aml);
            aml = remaining_aml;
            package_element_list.push(package_element);
        }
        Self(package_element_list)
    }
}

impl Reader<'_> for PackageElementList {
    fn length(&self) -> usize {
        self.0
            .iter()
            .map(|package_element| package_element.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        if aml.is_empty() {
            true
        } else {
            PackageElement::matches(aml)
        }
    }
}

