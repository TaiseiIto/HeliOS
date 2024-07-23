use {
    core::fmt,
    super::{
        Operand,
        Reader,
        SubtractOp,
        Target,
    },
};

/// # DefSubtract
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefSubtract {
    subtract_op: SubtractOp,
    operands: [Operand; 2],
    target: Target,
}

impl fmt::Debug for DefSubtract {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            subtract_op,
            operands,
            target,
        } = self;
        formatter
            .debug_tuple("DefSubtract")
            .field(subtract_op)
            .field(&operands[0])
            .field(&operands[1])
            .field(target)
            .finish()
    }
}

impl From<&[u8]> for DefSubtract {
    fn from(aml: &[u8]) -> Self {
        let (subtract_op, aml): (SubtractOp, &[u8]) = SubtractOp::read(aml);
        let (operand0, aml): (Operand, &[u8]) = Operand::read(aml);
        let (operand1, aml): (Operand, &[u8]) = Operand::read(aml);
        let operands: [Operand; 2] = [operand0, operand1];
        let (target, aml): (Target, &[u8]) = Target::read(aml);
        Self {
            subtract_op,
            operands,
            target,
        }
    }
}

impl Reader<'_> for DefSubtract {
    fn length(&self) -> usize {
        let Self {
            subtract_op,
            operands,
            target,
        } = self;
        subtract_op.length()
        + operands
            .as_slice()
            .iter()
            .map(|operand| operand.length())
            .sum::<usize>()
        + target.length()
    }

    fn matches(aml: &[u8]) -> bool {
        SubtractOp::matches(aml)
    }
}

