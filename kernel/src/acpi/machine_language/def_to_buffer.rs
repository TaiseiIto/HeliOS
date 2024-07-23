use {
    core::fmt,
    super::{
        Operand,
        Reader,
        TO_BUFFER_OP,
        Target,
        ToBufferOp,
    },
};

/// # DefToBuffer
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 ExpressionOpcodes Encoding
pub struct DefToBuffer {
    to_buffer_op: ToBufferOp,
    operand: Operand,
    target: Target,
}

impl fmt::Debug for DefToBuffer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            to_buffer_op,
            operand,
            target,
        } = self;
        formatter
            .debug_tuple("DefToBuffer")
            .field(to_buffer_op)
            .field(operand)
            .field(target)
            .finish()
    }
}

impl From<&[u8]> for DefToBuffer {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (to_buffer_op, aml): (ToBufferOp, &[u8]) = ToBufferOp::read(aml);
        let (operand, aml): (Operand, &[u8]) = Operand::read(aml);
        let (target, aml): (Target, &[u8]) = Target::read(aml);
        Self {
            to_buffer_op,
            operand,
            target,
        }
    }
}

impl Reader<'_> for DefToBuffer {
    fn length(&self) -> usize {
        let Self {
            to_buffer_op,
            operand,
            target,
        } = self;
        to_buffer_op.length()
        + operand.length()
        + target.length()
    }

    fn matches(aml: &[u8]) -> bool {
        ToBufferOp::matches(aml)
    }
}

