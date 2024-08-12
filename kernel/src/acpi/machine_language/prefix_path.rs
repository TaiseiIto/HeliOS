use {
    alloc::vec::Vec,
    super::Circumflex,
};

/// # PrefixPath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[matching_elements = 0]
pub struct PrefixPath(Vec<Circumflex>);

