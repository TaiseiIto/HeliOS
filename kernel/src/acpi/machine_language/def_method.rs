use {
    core::fmt,
    super::{
        MethodFlags,
        MethodOp,
        NameString,
        PkgLength,
        TermList,
    },
};

/// # DefMethod
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct DefMethod {
    method_op: MethodOp,
    pkg_length: PkgLength,
    name_string: NameString,
    method_flags: MethodFlags,
    term_list: TermList,
}

impl DefMethod {
    pub fn length(&self) -> usize {
        let Self {
            method_op,
            pkg_length,
            name_string,
            method_flags,
            term_list,
        } = self;
        method_op.length()
        + pkg_length.length()
        + name_string.length()
        + method_flags.length()
        + term_list.length()
    }
}

impl fmt::Debug for DefMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            method_op,
            pkg_length,
            name_string,
            method_flags,
            term_list,
        } = self;
        formatter
            .debug_tuple("DefMethod")
            .field(method_op)
            .field(pkg_length)
            .field(name_string)
            .field(method_flags)
            .field(term_list)
            .finish()
    }
}

impl From<&[u8]> for DefMethod {
    fn from(aml: &[u8]) -> Self {
        let method_op: MethodOp = aml.into();
        let aml: &[u8] = &aml[method_op.length()..];
        let pkg_length: PkgLength = aml.into();
        let aml: &[u8] = &aml[pkg_length.length()..pkg_length.pkg_length()];
        let name_string: NameString = aml.into();
        let aml: &[u8] = &aml[name_string.length()..];
        let method_flags: MethodFlags = aml.into();
        let aml: &[u8] = &aml[method_flags.length()..];
        let term_list: TermList = aml.into();
        let aml: &[u8] = &aml[term_list.length()..];
        Self {
            method_op,
            pkg_length,
            name_string,
            method_flags,
            term_list,
        }
    }
}

