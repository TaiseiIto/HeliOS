use super::{
    NameString,
    PkgLength,
    ScopeOp,
};

/// # DefScope
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
#[derive(Debug)]
pub struct DefScope {
    scope_op: ScopeOp,
    pkg_length: PkgLength,
    name_string: NameString,
}

impl From<&[u8]> for DefScope {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first().unwrap() {
            0x10 => {
                let scope_op: ScopeOp = bytes.into();
                let pkg_length: PkgLength = (&bytes[scope_op.length()..]).into();
                let bytes: &[u8] = &bytes[scope_op.length() + pkg_length.length()..pkg_length.pkg_length()];
                let name_string: NameString = bytes.into();
                Self {
                    scope_op,
                    pkg_length,
                    name_string,
                }
            },
            unknown_byte => panic!("Unknown byte {:#x?}", unknown_byte),
        }
    }
}

