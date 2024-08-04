use super::{
    Operand,
    Target,
    ToBufferOp,
};

/// # DefToBuffer
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 ExpressionOpcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub struct DefToBuffer(
    ToBufferOp,
    Operand,
    Target,
);

