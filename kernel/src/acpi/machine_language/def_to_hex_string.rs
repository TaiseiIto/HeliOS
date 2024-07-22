use {
    core::fmt,
    super::{
        Operand,
        Reader,
        Target,
        ToHexStringOp,
    },
};

/// # DefToHexString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefToHexString {
    to_hex_string_op: ToHexStringOp,
    operand: Operand,
    target: Target,
}

impl fmt::Debug for DefToHexString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            to_hex_string_op,
            operand,
            target,
        } = self;
        formatter
            .debug_tuple("DefToHexString")
            .field(to_hex_string_op)
            .field(operand)
            .field(target)
            .finish()
    }
}

impl From<&[u8]> for DefToHexString {
    fn from(aml: &[u8]) -> Self {
        let (to_hex_string_op, aml): (ToHexStringOp, &[u8]) = ToHexStringOp::read(aml);
        let (operand, aml): (Operand, &[u8]) = Operand::read(aml);
        let (target, aml): (Target, &[u8]) = Target::read(aml);
        Self {
            to_hex_string_op,
            operand,
            target,
        }
    }
}

impl Reader<'_> for DefToHexString {
    fn length(&self) -> usize {
        let Self {
            to_hex_string_op,
            operand,
            target,
        } = self;
        to_hex_string_op.length()
        + operand.length()
        + target.length()
    }
}

