use {
    core::fmt,
    super::{
        PkgLength,
        Predicate,
        Reader,
        TermList,
        WhileOp,
    },
};

/// # DefWhile
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
pub struct DefWhile {
    while_op: WhileOp,
    pkg_length: PkgLength,
    predicate: Predicate,
    term_list: TermList,
}

impl fmt::Debug for DefWhile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            while_op,
            pkg_length,
            predicate,
            term_list,
        } = self;
        formatter
            .debug_tuple("DefWhile")
            .field(while_op)
            .field(pkg_length)
            .field(predicate)
            .field(term_list)
            .finish()
    }
}

impl From<&[u8]> for DefWhile {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (while_op, aml): (WhileOp, &[u8]) = WhileOp::read(aml);
        let (pkg_length, aml): (PkgLength, &[u8]) = PkgLength::read(aml);
        let (predicate, aml): (Predicate, &[u8]) = Predicate::read(aml);
        let (term_list, _aml): (TermList, &[u8]) = TermList::read(aml);
        Self {
            while_op,
            pkg_length,
            predicate,
            term_list,
        }
    }
}

impl Reader<'_> for DefWhile {
    fn length(&self) -> usize {
        let Self {
            while_op,
            pkg_length,
            predicate,
            term_list,
        } = self;
        while_op.length()
        + pkg_length.length()
        + predicate.length()
        + term_list.length()
    }

    fn matches(aml: &[u8]) -> bool {
        WhileOp::matches(aml)
    }
}

