use {
    core::{
        fmt,
        slice,
    },
    super::{
        DefOpRegion,
        EXT_OP_PREFIX,
        OP_REGION_OP,
    },
};

/// # NamedObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.2 Named Objects Encoding
pub enum NamedObj {
    DefOpRegion(DefOpRegion),
}

impl fmt::Debug for NamedObj {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
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
                OP_REGION_OP => {
                    let def_op_region: DefOpRegion = aml.into();
                    Self::DefOpRegion(def_op_region)
                },
                unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
            },
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

