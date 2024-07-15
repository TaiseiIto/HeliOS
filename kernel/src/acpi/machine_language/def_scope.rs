use {
    core::fmt,
    super::{
        NameString,
        PkgLength,
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

impl DefScope {
    pub fn length(&self) -> usize {
        let Self {
            scope_op,
            pkg_length,
            name_string,
            term_list,
        } = self;
        scope_op.length() + pkg_length.length() + name_string.length() + term_list.length()
    }
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
        match *aml.first().unwrap() {
            SCOPE_OP => {
                let scope_op: ScopeOp = aml.into();
                let pkg_length: PkgLength = (&aml[scope_op.length()..]).into();
                let aml: &[u8] = &aml[scope_op.length() + pkg_length.length()..pkg_length.pkg_length()];
                let name_string: NameString = aml.into();
                let aml: &[u8] = &aml[name_string.length()..];
                let term_list: TermList = aml.into();
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

