use core::fmt;

/// # DataObject
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.3 Data Objects Encoding
pub enum DataObject {
    ComputationalData,
    DefPackage,
    DefVarPackage,
}

impl fmt::Debug for DataObject {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ComputationalData => write!(formatter, "DataObject::ComputationalData"),
            Self::DefPackage => write!(formatter, "DataObject::DefPackage"),
            Self::DefVarPackage => write!(formatter, "DataObject::DefVarPackage"),
        }
    }
}

impl From<&[u8]> for DataObject {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

