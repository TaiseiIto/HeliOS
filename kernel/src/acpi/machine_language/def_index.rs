use {
    core::fmt,
    super::{
        IndexOp,
        Reader,
    },
};

/// # DefIndex
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub struct DefIndex {
    index_op: IndexOp,
}

impl fmt::Debug for DefIndex {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            index_op,
        } = self;
        formatter
            .debug_tuple("DefIndex")
            .field(index_op)
            .finish()
    }
}

impl From<&[u8]> for DefIndex {
    fn from(aml: &[u8]) -> Self {
        let (index_op, aml): (IndexOp, &[u8]) = IndexOp::read(aml);
        unimplemented!()
    }
}

impl Reader<'_> for DefIndex {
    fn length(&self) -> usize {
        let Self {
            index_op,
        } = self;
        index_op.length()
    }

    fn matches(aml: &[u8]) -> bool {
        IndexOp::matches(aml)
    }
}

