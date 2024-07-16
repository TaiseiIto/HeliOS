use {
    alloc::string::String,
    core::fmt,
    crate::{
        com2_print,
        com2_println,
    },
    super::{
        FieldFlags,
        FieldOp,
        NameString,
        PkgLength,
    },
};

/// # DefField
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub struct DefField {
    field_op: FieldOp,
    pkg_length: PkgLength,
    name_string: NameString,
    field_flags: FieldFlags,
}

impl DefField {
    pub fn length(&self) -> usize {
        let Self {
            field_op,
            pkg_length,
            name_string,
            field_flags,
        } = self;
        field_op.length()
        + pkg_length.length()
        + name_string.length()
        + field_flags.length()
    }
}

impl fmt::Debug for DefField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            field_op,
            pkg_length,
            name_string,
            field_flags,
        } = self;
        formatter
            .debug_tuple("DefField")
            .field(field_op)
            .field(pkg_length)
            .field(name_string)
            .field(field_flags)
            .finish()
    }
}

impl From<&[u8]> for DefField {
    fn from(aml: &[u8]) -> Self {
        let field_op: FieldOp = aml.into();
        let pkg_length: PkgLength = (&aml[field_op.length()..]).into();
        let aml: &[u8] = &aml[field_op.length() + pkg_length.length()..pkg_length.pkg_length()];
        let name_string: NameString = aml.into();
        let name: String = (&name_string).into();
        com2_println!("name = {:#x?}", name);
        let aml: &[u8] = &aml[name_string.length()..];
        let field_flags: FieldFlags = aml.into();
        let aml: &[u8] = &aml[field_flags.length()..];
        unimplemented!()
    }
}

