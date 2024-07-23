use {
    core::fmt,
    super::{
        FieldFlags,
        FieldList,
        FieldOp,
        NameString,
        PkgLength,
        Reader,
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
        let (field_op, aml): (FieldOp, &[u8]) = FieldOp::read(aml);
        let (pkg_length, aml): (PkgLength, &[u8]) = PkgLength::read(aml);
        let (name_string, aml): (NameString, &[u8]) = NameString::read(aml);
        let (field_flags, aml): (FieldFlags, &[u8]) = FieldFlags::read(aml);
        let (field_list, aml): (FieldList, &[u8]) = FieldList::read(aml);
        Self {
            field_op,
            pkg_length,
            name_string,
            field_flags,
            field_list,
        }
    }
}

impl Reader<'_> for DefField {
    fn length(&self) -> usize {
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

    fn matches(aml: &[u8]) -> bool {
        FieldOp::matches(aml)
    }
}

