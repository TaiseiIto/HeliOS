use {
    core::fmt,
    crate::{
        com2_print,
        com2_println,
    },
    super::{
        FieldFlags,
        FieldList,
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
    field_list: FieldList,
}

impl DefField {
    pub fn length(&self) -> usize {
        let Self {
            field_op,
            pkg_length,
            name_string,
            field_flags,
            field_list,
        } = self;
        field_op.length()
        + pkg_length.length()
        + name_string.length()
        + field_flags.length()
        + field_list.length()
    }
}

impl fmt::Debug for DefField {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            field_op,
            pkg_length,
            name_string,
            field_flags,
            field_list,
        } = self;
        formatter
            .debug_tuple("DefField")
            .field(field_op)
            .field(pkg_length)
            .field(name_string)
            .field(field_flags)
            .field(field_list)
            .finish()
    }
}

impl From<&[u8]> for DefField {
    fn from(aml: &[u8]) -> Self {
        com2_println!("aml = {:#x?}", aml);
        let field_op: FieldOp = aml.into();
        com2_println!("field_op = {:#x?}", field_op);
        let aml: &[u8] = &aml[field_op.length()..];
        com2_println!("aml = {:#x?}", aml);
        let pkg_length: PkgLength = aml.into();
        com2_println!("pkg_length = {:#x?}", pkg_length);
        let aml: &[u8] = &aml[pkg_length.length()..pkg_length.pkg_length()];
        com2_println!("aml = {:#x?}", aml);
        let name_string: NameString = aml.into();
        com2_println!("name_string = {:#x?}", name_string);
        let aml: &[u8] = &aml[name_string.length()..];
        com2_println!("aml = {:#x?}", aml);
        let field_flags: FieldFlags = aml.into();
        com2_println!("field_flags = {:#x?}", field_flags);
        let aml: &[u8] = &aml[field_flags.length()..];
        com2_println!("aml = {:#x?}", aml);
        let field_list: FieldList = aml.into();
        com2_println!("field_list = {:#x?}", field_list);
        Self {
            field_op,
            pkg_length,
            name_string,
            field_flags,
            field_list,
        }
    }
}

