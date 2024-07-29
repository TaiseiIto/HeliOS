use {
    core::fmt,
    super::{
        ArgObject,
        Reader,
        ReturnOp,
    },
};

/// # DefReturn
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
pub struct DefReturn {
    return_op: ReturnOp,
    arg_object: ArgObject,
}

impl fmt::Debug for DefReturn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            return_op,
            arg_object,
        } = self;
        formatter
            .debug_tuple("DefReturn")
            .field(return_op)
            .field(arg_object)
            .finish()
    }
}

impl From<&[u8]> for DefReturn {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (return_op, aml): (ReturnOp, &[u8]) = ReturnOp::read(aml);
        let (arg_object, aml): (ArgObject, &[u8]) = ArgObject::read(aml);
        Self {
            return_op,
            arg_object,
        }
    }
}

impl Reader<'_> for DefReturn {
    fn length(&self) -> usize {
        let Self {
            return_op,
            arg_object,
        } = self;
        return_op.length() + arg_object.length()
    }

    fn matches(aml: &[u8]) -> bool {
        ReturnOp::matches(aml)
    }
}

