use {
    core::fmt,
    crate::{
        com2_print,
        com2_println,
    },
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
                com2_println!("aml = {:#x?}", aml);
                let scope_op: ScopeOp = aml.into();
                com2_println!("scope_op = {:#x?}", scope_op);
                let aml: &[u8] = &aml[scope_op.length()..];
                com2_println!("aml = {:#x?}", aml);
                let pkg_length: PkgLength = (aml).into();
                com2_println!("pkg_length = {:#x?}", pkg_length);
                let aml: &[u8] = &aml[pkg_length.length()..pkg_length.pkg_length()];
                com2_println!("aml = {:#x?}", aml);
                let name_string: NameString = aml.into();
                com2_println!("name_string = {:#x?}", name_string);
                let aml: &[u8] = &aml[name_string.length()..];
                com2_println!("aml = {:#x?}", aml);
                let term_list: TermList = aml.into();
                com2_println!("term_list = {:#x?}", term_list);
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

