use {
    core::fmt,
    super::{
        IndexOp,
        Reader,
        SuperName,
    },
};

/// # DefIncrement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefIncrement {
    index_op: IndexOp,
    super_name: SuperName,
}

impl fmt::Debug for DefIncrement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            index_op,
            super_name,
        } = self;
        formatter
            .debug_tuple("DefIncrement")
            .field(index_op)
            .field(super_name)
            .finish()
    }
}

impl From<&[u8]> for DefIncrement {
    fn from(aml: &[u8]) -> Self {
        let (index_op, aml): (IndexOp, &[u8]) = IndexOp::read(aml);
        let (super_name, aml): (SuperName, &[u8]) = SuperName::read(aml);
        Self {
            index_op,
            super_name,
        }
    }
}

impl Reader<'_> for DefIncrement {
    fn length(&self) -> usize {
        let Self {
            index_op,
            super_name,
        } = self;
        index_op.length() + super_name.length()
    }

    fn matches(aml: &[u8]) -> bool {
        IndexOp::matches(aml)
    }
}

