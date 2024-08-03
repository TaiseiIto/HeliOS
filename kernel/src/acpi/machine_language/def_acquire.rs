use {
    core::fmt,
    super::{
        AcquireOp,
        MutexObject,
        Reader,
        Timeout,
    },
};

/// # DefAcquire
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefAcquire {
    acquire_op: AcquireOp,
    mutex_object: MutexObject,
    timeout: Timeout,
}

impl fmt::Debug for DefAcquire {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            acquire_op,
            mutex_object,
            timeout,
        } = self;
        formatter
            .debug_tuple("DefAcquire")
            .field(acquire_op)
            .field(mutex_object)
            .field(timeout)
            .finish()
    }
}

impl From<&[u8]> for DefAcquire {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (acquire_op, aml): (AcquireOp, &[u8]) = AcquireOp::read(aml);
        let (mutex_object, aml): (MutexObject, &[u8]) = MutexObject::read(aml);
        let (timeout, aml): (Timeout, &[u8]) = Timeout::read(aml);
        Self {
            acquire_op,
            mutex_object,
            timeout,
        }
    }
}

impl Reader<'_> for DefAcquire {
    fn length(&self) -> usize {
        let Self {
            acquire_op,
            mutex_object,
            timeout,
        } = self;
        acquire_op.length()
        + mutex_object.length()
        + timeout.length()
    }

    fn matches(aml: &[u8]) -> bool {
        AcquireOp::matches(aml)
    }
}
