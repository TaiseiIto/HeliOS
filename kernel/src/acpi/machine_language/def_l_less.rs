use {
    core::fmt,
    super::{
        LLessOp,
        Operand,
        Reader,
    },
};

/// # DefLLess
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefLLess {
    l_less_op: LLessOp,
    operand: [Operand; 2],
}

impl fmt::Debug for DefLLess {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            l_less_op,
            operand,
        } = self;
        formatter
            .debug_tuple("DefLLess")
            .field(l_less_op)
            .field(&operand[0])
            .field(&operand[1])
            .finish()
    }
}

impl From<&[u8]> for DefLLess {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (l_less_op, aml): (LLessOp, &[u8]) = LLessOp::read(aml);
        let (operand0, aml): (Operand, &[u8]) = Operand::read(aml);
        let (operand1, _aml): (Operand, &[u8]) = Operand::read(aml);
        let operand: [Operand; 2] = [operand0, operand1];
        Self {
            l_less_op,
            operand,
        }
    }
}

impl Reader<'_> for DefLLess {
    fn length(&self) -> usize {
        let Self {
            l_less_op,
            operand,
        } = self;
        l_less_op.length() + operand
            .as_slice()
            .iter()
            .map(|operand| operand.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        LLessOp::matches(aml)
    }
}

