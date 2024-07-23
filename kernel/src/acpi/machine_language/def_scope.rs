use {
    core::fmt,
    super::{
        NameString,
        PkgLength,
        Reader,
        ScopeOp,
        SCOPE_OP,
        TermList,
    },
};

/// # DefScope
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
pub struct DefScope {
    scope_op: ScopeOp,
    pkg_length: PkgLength,
    name_string: NameString,
    term_list: TermList,
}

impl fmt::Debug for DefScope {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            scope_op,
            pkg_length,
            name_string,
            term_list,
        } = self;
        formatter
            .debug_tuple("DefScope")
            .field(scope_op)
            .field(pkg_length)
            .field(name_string)
            .field(term_list)
            .finish()
    }
}

impl From<&[u8]> for DefScope {
    fn from(aml: &[u8]) -> Self {
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        match *aml.first().unwrap() {
            SCOPE_OP => {
                let (scope_op, aml): (ScopeOp, &[u8]) = ScopeOp::read(aml);
                let (pkg_length, aml): (PkgLength, &[u8]) = PkgLength::read(aml);
                let (name_string, aml): (NameString, &[u8]) = NameString::read(aml);
                let (term_list, aml): (TermList, &[u8]) = TermList::read(aml);
                Self {
                    scope_op,
                    pkg_length,
                    name_string,
                    term_list,
                }
            },
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

impl Reader<'_> for DefScope {
    fn length(&self) -> usize {
        let Self {
            scope_op,
            pkg_length,
            name_string,
            term_list,
        } = self;
        scope_op.length() + pkg_length.length() + name_string.length() + term_list.length()
    }

    fn matches(aml: &[u8]) -> bool {
        ScopeOp::matches(aml)
    }
}

