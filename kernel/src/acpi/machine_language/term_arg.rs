use core::fmt;

/// # TermArg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum TermArg {
    ExpressionOpcode,
    DataObject,
    ArgObj,
    LocalObj,
}

impl TermArg {
    pub fn length(&self) -> usize {
        unimplemented!()
    }
}

impl fmt::Debug for TermArg {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpressionOpcode => write!(formatter, "TermArg::ExpressionOpcode"),
            Self::DataObject => write!(formatter, "TermArg::DataObject"),
            Self::ArgObj => write!(formatter, "TermArg::ArgObj"),
            Self::LocalObj => write!(formatter, "TermArg::LocalObj"),
        }
    }
}

impl From<&[u8]> for TermArg {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

