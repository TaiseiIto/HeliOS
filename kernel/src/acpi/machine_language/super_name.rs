use {
    core::fmt,
    super::{
        Reader,
        SimpleName,
    },
};

/// # SuperName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum SuperName {
    SimpleName(SimpleName),
}

impl fmt::Debug for SuperName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SimpleName(simple_name) => formatter
                .debug_tuple("SuperName")
                .field(simple_name)
                .finish(),
        }
    }
}

impl From<&[u8]> for SuperName {
    fn from(aml: &[u8]) -> Self {
        if SimpleName::matches(aml) {
            Self::SimpleName(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for SuperName {
    fn length(&self) -> usize {
        match self {
            Self::SimpleName(simple_name) => simple_name.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        SimpleName::matches(aml)
    }
}

