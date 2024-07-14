/// # ACPI Machine Lnaguage
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(Debug)]
pub enum Symbol {
    Nothing,
}

impl From<&[u8]> for Symbol {
    fn from(term_list: &[u8]) -> Self {
        Self::Nothing
    }
}

