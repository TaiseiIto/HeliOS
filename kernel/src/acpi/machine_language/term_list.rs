use {
    acpi_machine_language::Reader,
    alloc::vec::Vec,
    crate::{
        com2_print,
        com2_println,
    },
    super::{
        TermObj,
        Reader,
    },
};

/// # TermList
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(Reader)]
pub struct TermList(Vec<TermObj>);

