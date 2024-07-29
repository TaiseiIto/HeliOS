use {
    core::fmt,
    super::{
        LNotOp,
        Operand,
        Reader,
    },
};

/// # DefLNot
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefLNot {
    l_not_op: LNotOp,
    operand: Operand,
}

impl fmt::Debug for DefLNot {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            l_not_op,
            operand,
        } = self;
        formatter
            .debug_tuple("DefLNot")
            .field(l_not_op)
            .field(operand)
            .finish()
    }
}

impl From<&[u8]> for DefLNot {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (l_not_op, aml): (LNotOp, &[u8]) = LNotOp::read(aml);
        let (operand, aml): (Operand, &[u8]) = Operand::read(aml);
        Self {
            l_not_op,
            operand,
        }
    }
}

impl Reader<'_> for DefLNot {
    fn length(&self) -> usize {
        let Self {
            l_not_op,
            operand,
        } = self;
        l_not_op.length() + operand.length()
    }

    fn matches(aml: &[u8]) -> bool {
        LNotOp::matches(aml)
    }
}

