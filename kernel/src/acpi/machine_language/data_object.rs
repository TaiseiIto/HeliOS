use {
    core::fmt,
    super::{
        ComputationalData,
        DefPackage,
        Reader,
    },
};

/// # DataObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub enum DataObject {
    ComputationalData(ComputationalData),
    DefPackage(DefPackage),
}

impl fmt::Debug for DataObject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("DataObject");
        match self {
            Self::ComputationalData(computational_data) => debug_tuple.field(computational_data),
            Self::DefPackage(def_package) => debug_tuple.field(def_package),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for DataObject {
    fn from(aml: &[u8]) -> Self {
        if ComputationalData::matches(aml) {
            Self::ComputationalData(aml.into())
        } else if DefPackage::matches(aml) {
            Self::DefPackage(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for DataObject {
    fn length(&self) -> usize {
        match self {
            Self::ComputationalData(computational_data) => computational_data.length(),
            Self::DefPackage(def_package) => def_package.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        ComputationalData::matches(aml)
        || DefPackage::matches(aml)
    }
}

