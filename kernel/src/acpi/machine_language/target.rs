use {
    core::fmt,
    super::{
        NullName,
        Reader,
        SuperName,
    },
};

/// # Target
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum Target {
    NullName(NullName),
    SuperName(SuperName),
}

impl fmt::Debug for Target {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("Target");
        match self {
            Self::NullName(null_name) => debug_tuple.field(null_name),
            Self::SuperName(super_name) => debug_tuple.field(super_name),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for Target {
    fn from(aml: &[u8]) -> Self {
        if NullName::matches(aml) {
            Self::NullName(aml.into())
        } else if SuperName::matches(aml) {
            Self::SuperName(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for Target {
    fn length(&self) -> usize {
        match self {
            Self::NullName(null_name) => null_name.length(),
            Self::SuperName(super_name) => super_name.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        NullName::matches(aml) || SuperName::matches(aml)
    }
}

