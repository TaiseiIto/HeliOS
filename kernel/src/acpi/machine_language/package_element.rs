use {
    core::fmt,
    super::{
        DataRefObject,
        NameString,
        Reader,
    },
};

/// # PackageElement
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub enum PackageElement {
    DataRefObject(DataRefObject),
    NameString(NameString),
}

impl fmt::Debug for PackageElement {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("PackageElement");
        match self {
            Self::DataRefObject(data_ref_object) => debug_tuple.field(data_ref_object),
            Self::NameString(name_string) => debug_tuple.field(name_string),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for PackageElement {
    fn from(aml: &[u8]) -> Self {
        if DataRefObject::matches(aml) {
            Self::DataRefObject(aml.into())
        } else if NameString::matches(aml) {
            Self::NameString(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for PackageElement {
    fn length(&self) -> usize {
        match self {
            Self::DataRefObject(data_ref_object) => data_ref_object.length(),
            Self::NameString(name_string) => name_string.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        DataRefObject::matches(aml)
        || NameString::matches(aml)
    }
}

