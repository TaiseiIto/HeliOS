use {
    core::fmt,
    super::{
        DerefOfOp,
        Reader,
    },
};

/// # DefDerefOf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefDerefOf {
    deref_of_op: DerefOfOp,
}

impl fmt::Debug for DefDerefOf {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            deref_of_op,
        } = self;
        formatter
            .debug_tuple("DefDerefOf")
            .field(deref_of_op)
            .finish()
    }
}

impl From<&[u8]> for DefDerefOf {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (deref_of_op, aml): (DerefOfOp, &[u8]) = DerefOfOp::read(aml);
        unimplemented!()
    }
}

impl Reader<'_> for DefDerefOf {
    fn length(&self) -> usize {
        let Self {
            deref_of_op,
        } = self;
        deref_of_op.length()
    }

    fn matches(aml: &[u8]) -> bool {
        DerefOfOp::matches(aml)
    }
}

