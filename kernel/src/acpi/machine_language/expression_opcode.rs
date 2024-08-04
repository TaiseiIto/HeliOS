use super::{
    DefAcquire,
    DefBuffer,
    DefDerefOf,
    DefIncrement,
    DefIndex,
    DefLEqual,
    DefLLess,
    DefLNot,
    DefPackage,
    DefSizeOf,
    DefStore,
    DefSubtract,
    DefToBuffer,
    DefToHexString,
};

/// # ExpressionOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
#[derive(acpi_machine_language::Reader)]
pub enum ExpressionOpcode {
    DefAcquire(DefAcquire),
    DefBuffer(DefBuffer),
    DefDerefOf(DefDerefOf),
    DefIncrement(DefIncrement),
    DefIndex(DefIndex),
    DefLEqual(DefLEqual),
    DefLLess(DefLLess),
    DefLNot(DefLNot),
    DefPackage(DefPackage),
    DefSizeOf(DefSizeOf),
    DefStore(DefStore),
    DefSubtract(DefSubtract),
    DefToBuffer(DefToBuffer),
    DefToHexString(DefToHexString),
}

