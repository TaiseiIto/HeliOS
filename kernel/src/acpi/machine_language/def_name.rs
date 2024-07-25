use {
    core::fmt,
    super::{
        NameOp,
        NameString,
        Reader,
    },
};

/// # NameSpaceModifierObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
pub struct DefName {
    name_op: NameOp,
    name_string: NameString,
}

impl fmt::Debug for DefName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            name_op,
            name_string,
        } = self;
        formatter
            .debug_tuple("DefName")
            .field(name_op)
            .field(name_string)
            .finish()
    }
}

impl From<&[u8]> for DefName {
    fn from(aml: &[u8]) -> Self {
        let (name_op, aml): (NameOp, &[u8]) = NameOp::read(aml);
        let (name_string, aml): (NameString, &[u8]) = NameString::read(aml);
        unimplemented!()
    }
}

impl Reader<'_> for DefName {
    fn length(&self) -> usize {
        let Self {
            name_op,
            name_string,
        } = self;
        name_op.length()
        + name_string.length()
    }

    fn matches(aml: &[u8]) -> bool {
        NameOp::matches(aml)
    }
}

