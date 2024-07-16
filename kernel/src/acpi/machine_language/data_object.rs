use {
    core::fmt,
    super::{
        ComputationalData,
        ONE_OP,
        WORD_PREFIX,
    },
};

/// # DataObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub enum DataObject {
    ComputationalData(ComputationalData),
    DefPackage,
    DefVarPackage,
}

impl DataObject {
    pub fn length(&self) -> usize {
        match self {
            Self::ComputationalData(computational_data) => computational_data.length(),
            Self::DefPackage => unimplemented!(),
            Self::DefVarPackage => unimplemented!(),
        }
    }
}

impl fmt::Debug for DataObject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComputationalData(computational_data) => formatter
                .debug_tuple("DataObject")
                .field(computational_data)
                .finish(),
            Self::DefPackage => write!(formatter, "DataObject::DefPackage"),
            Self::DefVarPackage => write!(formatter, "DataObject::DefVarPackage"),
        }
    }
}

impl From<&[u8]> for DataObject {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            ONE_OP | WORD_PREFIX => Self::ComputationalData(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

