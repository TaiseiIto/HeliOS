use super::{
    MethodFlags,
    MethodOp,
    NameString,
    PkgLength,
    TermList,
};

/// # DefMethod
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefMethod(
    MethodOp,
    PkgLength,
    NameString,
    MethodFlags,
    TermList,
);

