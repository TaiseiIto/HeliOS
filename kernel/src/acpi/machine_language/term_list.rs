use {
    alloc::vec::Vec,
    super::TermObj,
};

/// # TermList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(Debug)]
pub struct TermList(Vec<TermObj>);

impl TermList {
    pub fn length(&self) -> usize {
        self.0
            .iter()
            .map(|term_obj| term_obj.length())
            .sum::<usize>()
    }
}

impl From<&[u8]> for TermList {
    fn from(aml: &[u8]) -> Self {
        let mut aml: &[u8] = aml;
        let mut term_list: Vec<TermObj> = Vec::new();
        while !aml.is_empty() {
            let term_obj: TermObj = aml.into();
            aml = &aml[term_obj.length()..];
            term_list.push(term_obj);
        }
        Self(term_list)
    }
}

