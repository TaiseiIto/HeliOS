use {
    core::fmt,
    super::{
        Reader,
        ElseOp,
        PkgLength,
        TermList,
    },
};

/// # DefElse
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
pub enum DefElse {
    Nothing,
    ElseOpPkgLengthTermList {
        else_op: ElseOp,
        pkg_length: PkgLength,
        term_list: TermList,
    },
}

impl fmt::Debug for DefElse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Nothing => write!(formatter, "DefElse"),
            Self::ElseOpPkgLengthTermList {
                else_op,
                pkg_length,
                term_list,
            } => formatter
                .debug_tuple("DefElse")
                .field(else_op)
                .field(pkg_length)
                .field(term_list)
                .finish()
        }
    }
}

impl From<&[u8]> for DefElse {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        if aml.is_empty() {
            Self::Nothing
        } else {
            let (else_op, aml): (ElseOp, &[u8]) = ElseOp::read(aml);
            let (pkg_length, aml): (PkgLength, &[u8]) = PkgLength::read(aml);
            let (term_list, aml): (TermList, &[u8]) = TermList::read(aml);
            Self::ElseOpPkgLengthTermList {
                else_op,
                pkg_length,
                term_list,
            }
        }
    }
}

impl Reader<'_> for DefElse {
    fn length(&self) -> usize {
        match self {
            Self::Nothing => 0,
            Self::ElseOpPkgLengthTermList {
                else_op,
                pkg_length,
                term_list,
            } => else_op.length()
                + pkg_length.length()
                + term_list.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        if aml.is_empty() {
            true
        } else {
            ElseOp::matches(aml)
        }
    }
}

