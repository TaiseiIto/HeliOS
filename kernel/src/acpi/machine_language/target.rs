use {
    core::fmt,
    super::{
        NULL_NAME,
        NullName,
        Reader,
    },
};

/// # Target
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum Target {
    NullName(NullName),
}

impl fmt::Debug for Target {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NullName(null_name) => formatter
                .debug_tuple("Target")
                .field(null_name)
                .finish(),
        }
    }
}

impl From<&[u8]> for Target {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            NULL_NAME => Self::NullName(aml.into()),
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

impl Reader<'_> for Target {
    fn length(&self) -> usize {
        match self {
            Self::NullName(null_name) => null_name.length(),
        }
    }
}

