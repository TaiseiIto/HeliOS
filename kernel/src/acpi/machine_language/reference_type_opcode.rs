use {
    core::fmt,
    super::{
        DefIndex,
        Reader,
    },
};

/// # ReferenceTypeOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub enum ReferenceTypeOpcode {
    DefIndex(DefIndex),
}

impl fmt::Debug for ReferenceTypeOpcode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("ReferenceTypeOpcode");
        match self {
            Self::DefIndex(def_index) => debug_tuple.field(def_index),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for ReferenceTypeOpcode {
    fn from(aml: &[u8]) -> Self {
        if DefIndex::matches(aml) {
            Self::DefIndex(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for ReferenceTypeOpcode {
    fn length(&self) -> usize {
        match self {
            Self::DefIndex(def_index) => def_index.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        DefIndex::matches(aml)
    }
}

