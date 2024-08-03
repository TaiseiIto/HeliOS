use {
    alloc::vec::Vec,
    super::AsciiChar,
};

/// # AsciiCharList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
#[derive(acpi_machine_language::Reader)]
#[matches = "always_true"]
pub struct AsciiCharList(Vec<AsciiChar>);

