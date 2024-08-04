use super::{
    DefElse,
    IfOp,
    PkgLength,
    Predicate,
    TermList,
};

/// # DefIfElse
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefIfElse(
    IfOp,
    PkgLength,
    Predicate,
    TermList,
    DefElse,
);

