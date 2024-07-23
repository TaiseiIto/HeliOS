use {
    core::fmt,
    super::{
        Reader,
        SizeOfOp,
        SuperName,
    },
};

/// # DefSizeOf
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefSizeOf {
    size_of_op: SizeOfOp,
    super_name: SuperName,
}

impl fmt::Debug for DefSizeOf {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            size_of_op,
            super_name,
        } = self;
        formatter
            .debug_tuple("DefSizeOf")
            .field(size_of_op)
            .field(super_name)
            .finish()
    }
}

impl From<&[u8]> for DefSizeOf {
    fn from(aml: &[u8]) -> Self {
        let (size_of_op, aml): (SizeOfOp, &[u8]) = SizeOfOp::read(aml);
        let (super_name, aml): (SuperName, &[u8]) = SuperName::read(aml);
        Self {
            size_of_op,
            super_name,
        }
    }
}

impl Reader<'_> for DefSizeOf {
    fn length(&self) -> usize {
        let Self {
            size_of_op,
            super_name,
        } = self;
        size_of_op.length() + super_name.length()
    }

    fn matches(aml: &[u8]) -> bool {
        true
    }
}

