use {
    alloc::vec::Vec,
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
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("DefSubtract");
        let Self {
            subtract_op,
            operands,
            target,
        } = self;
        debug_tuple.field(subtract_op);
        operands
            .as_slice()
            .iter()
            .for_each(|operand| {
                debug_tuple.field(operand);
            });
        debug_tuple
            .field(target)
            .finish()
    }
}

impl From<&[u8]> for DefSubtract {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (subtract_op, aml): (SubtractOp, &[u8]) = SubtractOp::read(aml);
        let (operands, aml): (Vec<Operand>, &[u8]) = (0..2)
            .fold((Vec::new(), aml), |(mut operands, aml), _| {
                let (operand, aml): (Operand, &[u8]) = Operand::read(aml);
                operands.push(operand);
                (operands, aml)
            });
        let operands: [Operand; 2] = operands
            .try_into()
            .unwrap();
        let (target, _aml): (Target, &[u8]) = Target::read(aml);
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

