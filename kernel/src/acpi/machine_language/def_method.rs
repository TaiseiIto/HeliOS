use {
    core::fmt,
    super::{
        MethodFlags,
        MethodOp,
        NameString,
        PkgLength,
        Reader,
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
        assert!(Self::matches(aml), "aml = {:#x?}", aml);
        let (method_op, aml): (MethodOp, &[u8]) = MethodOp::read(aml);
        let (pkg_length, aml): (PkgLength, &[u8]) = PkgLength::read(aml);
        let (name_string, aml): (NameString, &[u8]) = NameString::read(aml);
        let (method_flags, aml): (MethodFlags, &[u8]) = MethodFlags::read(aml);
        let (term_list, aml): (TermList, &[u8]) = TermList::read(aml);
        Self {
            method_op,
            pkg_length,
            name_string,
            method_flags,
            term_list,
        }
    }
}

impl Reader<'_> for DefMethod {
    fn length(&self) -> usize {
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

    fn matches(aml: &[u8]) -> bool {
        MethodOp::matches(aml)
    }
}

