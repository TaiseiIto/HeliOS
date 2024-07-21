use {
    core::fmt,
    super::{
        MethodOp,
        PkgLength,
        NameString,
    },
};

/// # DefMethod
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct DefMethod {
    method_op: MethodOp,
    pkg_length: PkgLength,
    name_string: NameString,
}

impl DefMethod {
    pub fn length(&self) -> usize {
        let Self {
            method_op,
            pkg_length,
            name_string,
        } = self;
        method_op.length()
        + pkg_length.length()
        + name_string.length()
    }
}

impl fmt::Debug for DefMethod {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            method_op,
            pkg_length,
            name_string,
        } = self;
        formatter
            .debug_tuple("DefMethod")
            .field(method_op)
            .field(pkg_length)
            .field(name_string)
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
        unimplemented!()
    }
}
