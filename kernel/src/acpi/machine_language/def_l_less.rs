use {
    alloc::vec::Vec,
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
    operands: [Operand; 2],
}

impl fmt::Debug for DefLLess {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("DefLLess");
        let Self {
            l_less_op,
            operands,
        } = self;
        debug_tuple.field(l_less_op);
        operands
            .as_slice()
            .iter()
            .for_each(|operand| {
                debug_tuple.field(operand);
            });
        debug_tuple.finish()
    }
}

impl From<&[u8]> for DefLLess {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (l_less_op, aml): (LLessOp, &[u8]) = LLessOp::read(aml);
        let (operands, aml): (Vec<Operand>, &[u8]) = (0..2)
            .fold((Vec::<Operand>::new(), aml), |(mut operands, aml), _| {
                let (operand, aml): (Operand, &[u8]) = Operand::read(aml);
                operands.push(operand);
                (operands, aml)
            });
        let operands: [Operand; 2] = operands
            .try_into()
            .unwrap();
        Self {
            l_less_op,
            operands,
        }
    }
}

impl Reader<'_> for DefLLess {
    fn length(&self) -> usize {
        let Self {
            l_less_op,
            operands,
        } = self;
        l_less_op.length() + operands
            .as_slice()
            .iter()
            .map(|operand| operand.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        LLessOp::matches(aml)
    }
}

