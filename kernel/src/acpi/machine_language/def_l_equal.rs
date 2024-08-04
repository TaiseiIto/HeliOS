use {
    alloc::vec::Vec,
    core::fmt,
    super::{
        LEqualOp,
        Operand,
        Reader,
    },
};

/// # DefLEqual
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefLEqual {
    l_equal_op: LEqualOp,
    operands: [Operand; 2],
}

impl fmt::Debug for DefLEqual {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("DefLEqual");
        let Self {
            l_equal_op,
            operands,
        } = self;
        debug_tuple.field(l_equal_op);
        operands
            .as_slice()
            .iter()
            .for_each(|operand| {
                debug_tuple.field(operand);
            });
        debug_tuple.finish()
    }
}

impl From<&[u8]> for DefLEqual {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (l_equal_op, aml): (LEqualOp, &[u8]) = LEqualOp::read(aml);
        let (operands, _aml): (Vec<Operand>, &[u8]) = (0..2)
            .fold((Vec::new(), aml), |(mut operands, aml), _| {
                let (operand, aml): (Operand, &[u8]) = Operand::read(aml);
                operands.push(operand);
                (operands, aml)
            });
        let operands: [Operand; 2] = operands
            .try_into()
            .unwrap();
        Self {
            l_equal_op,
            operands,
        }
    }
}

impl Reader<'_> for DefLEqual {
    fn length(&self) -> usize {
        let Self {
            l_equal_op,
            operands,
        } = self;
        l_equal_op.length() + operands
            .as_slice()
            .iter()
            .map(|operand| operand.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        LEqualOp::matches(aml)
    }
}

