use {
    core::fmt,
    super::{
        DataObject,
        ObjReference,
        Reader,
    },
};

/// # DataRefObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub enum DataRefObject {
    DataObject(DataObject),
    ObjReference(ObjReference),
}

impl fmt::Debug for DataRefObject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("DataRefObject");
        match self {
            Self::DataObject(data_object) => debug_tuple.field(data_object),
            Self::ObjReference(obj_reference) => debug_tuple.field(obj_reference),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for DataRefObject {
    fn from(aml: &[u8]) -> Self {
        if DataObject::matches(aml) {
            Self::DataObject(aml.into())
        } else if ObjReference::matches(aml) {
            Self::ObjReference(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for DataRefObject {
    fn length(&self) -> usize {
        match self {
            Self::DataObject(data_object) => data_object.length(),
            Self::ObjReference(obj_reference) => obj_reference.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        DataObject::matches(aml) || ObjReference::matches(aml)
    }
}

