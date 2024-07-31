use {
    core::fmt,
    super::{
        AcquireOp,
        Reader,
    },
};

/// # DefAcquire
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefAcquire {
    acquire_op: AcquireOp,
}

impl fmt::Debug for DefAcquire {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            acquire_op,
        } = self;
        formater
            .debug_tuple("DefAcquire")
            .field(acquire_op)
            .finish()
    }
}

impl From<&[u8]> for DefAcquire {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (acquire_op, aml): (AcquireOp, &[u8]) = AcquireOp::read(aml);
        unimplemented!()
    }
}

impl Reader<'_> for DefAcquire {
    fn length(&self) -> usize {
        let Self {
            acuiqre_op,
        } = self;
        acquire_op.length()
    }

    fn matches(aml: &[u8]) -> bool {
        AcquireOp::matches(aml)
    }
}

