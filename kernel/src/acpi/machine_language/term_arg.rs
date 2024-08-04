use super::{
    ArgObj,
    DataObject,
    ExpressionOpcode,
    LocalObj,
};

/// # TermArg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
#[derive(acpi_machine_language::Reader)]
pub enum TermArg {
    ExpressionOpcode(ExpressionOpcode),
    DataObject(DataObject),
    ArgObj(ArgObj),
    LocalObj(LocalObj),
}

