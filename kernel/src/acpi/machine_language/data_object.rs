use {
    core::fmt,
    super::{
        ComputationalData,
        Reader,
    },
};

/// # DataObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub enum DataObject {
    ComputationalData(ComputationalData),
}

impl fmt::Debug for DataObject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComputationalData(computational_data) => formatter
                .debug_tuple("DataObject")
                .field(computational_data)
                .finish(),
        }
    }
}

impl From<&[u8]> for DataObject {
    fn from(aml: &[u8]) -> Self {
        if ComputationalData::matches(aml) {
            Self::ComputationalData(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for DataObject {
    fn length(&self) -> usize {
        match self {
            Self::ComputationalData(computational_data) => computational_data.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        ComputationalData::matches(aml)
    }
}

