use {
    acpi_machine_language::Symbol,
    alloc::vec::Vec,
    super::TermObj,
};

/// # TermList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(Symbol)]
pub struct TermList(
    #[debug]
    Vec<TermObj>
);

