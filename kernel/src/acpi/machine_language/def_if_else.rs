use {
    core::fmt,
    super::{
        DefElse,
        IfOp,
        PkgLength,
        Predicate,
        Reader,
        TermList,
    },
};

/// # DefIfElse
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
pub struct DefIfElse {
    if_op: IfOp,
    pkg_length: PkgLength,
    predicate: Predicate,
    term_list: TermList,
    def_else: DefElse,
}

impl fmt::Debug for DefIfElse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            if_op,
            pkg_length,
            predicate,
            term_list,
            def_else,
        } = self;
        formatter
            .debug_tuple("DefIfElse")
            .field(if_op)
            .field(pkg_length)
            .field(predicate)
            .field(term_list)
            .field(def_else)
            .finish()
    }
}

impl From<&[u8]> for DefIfElse {
    fn from(aml: &[u8]) -> Self {
        let (if_op, aml): (IfOp, &[u8]) = IfOp::read(aml);
        let (pkg_length, aml): (PkgLength, &[u8]) = PkgLength::read(aml);
        let (predicate, aml): (Predicate, &[u8]) = Predicate::read(aml);
        let (term_list, aml): (TermList, &[u8]) = TermList::read(aml);
        let (def_else, aml): (DefElse, &[u8]) = DefElse::read(aml);
        Self {
            if_op,
            pkg_length,
            predicate,
            term_list,
            def_else,
        }
    }
}

impl Reader<'_> for DefIfElse {
    fn length(&self) -> usize {
        let Self {
            if_op,
            pkg_length,
            predicate,
            term_list,
            def_else,
        } = self;
        if_op.length()
        + pkg_length.length()
        + predicate.length()
        + term_list.length()
        + def_else.length()
    }

    fn matches(aml: &[u8]) -> bool {
        IfOp::matches(aml)
    }
}

