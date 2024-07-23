use {
    core::fmt,
    super::{
        PkgLength,
        Reader,
        WhileOp,
    },
};

/// # DefWhile
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
pub struct DefWhile {
    while_op: WhileOp,
    pkg_length: PkgLength,
}

impl fmt::Debug for DefWhile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            while_op,
            pkg_length,
        } = self;
        formatter
            .debug_tuple("DefWhile")
            .field(while_op)
            .field(pkg_length)
            .finish()
    }
}

impl From<&[u8]> for DefWhile {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (while_op, aml): (WhileOp, &[u8]) = WhileOp::read(aml);
        let (pkg_length, aml): (PkgLength, &[u8]) = PkgLength::read(aml);
        unimplemented!()
    }
}

impl Reader<'_> for DefWhile {
    fn length(&self) -> usize {
        let Self {
            while_op,
            pkg_length,
        } = self;
        while_op.length()
        + pkg_length.length()
    }

    fn matches(aml: &[u8]) -> bool {
        WhileOp::matches(aml)
    }
}

