use {
    core::{
        fmt,
        slice,
    },
    super::{
        DefField,
        DefMethod,
        DefOpRegion,
        EXT_OP_PREFIX,
        FIELD_OP,
        OP_REGION_OP,
    },
};

/// # NamedObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub enum NamedObj {
    DefField(DefField),
    DefMethod(DefMethod),
    DefOpRegion(DefOpRegion),
}

impl NamedObj {
    pub fn length(&self) -> usize {
        match self {
            Self::DefField(def_field) => def_field.length(),
            Self::DefMethod(def_method) => def_method.length(),
            Self::DefOpRegion(def_op_region) => def_op_region.length(),
        }
    }
}

impl fmt::Debug for NamedObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DefField(def_field) => formatter
                .debug_tuple("NamedObj")
                .field(def_field)
                .finish(),
            Self::DefMethod(def_method) => formatter
                .debug_tuple("NamedObj")
                .field(def_method)
                .finish(),
            Self::DefOpRegion(def_op_region) => formatter
                .debug_tuple("NamedObj")
                .field(def_op_region)
                .finish(),
        }
    }
}

impl From<&[u8]> for NamedObj {
    fn from(aml: &[u8]) -> Self {
        let mut aml_iterator: slice::Iter<u8> = aml.iter();
        match *aml_iterator.next().unwrap() {
            EXT_OP_PREFIX => match *aml_iterator.next().unwrap() {
                FIELD_OP => Self::DefField(aml.into()),
                OP_REGION_OP => Self::DefOpRegion(aml.into()),
                unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
            },
            METHOD_OP => Self::DefMethod(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

