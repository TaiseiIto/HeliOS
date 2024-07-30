use {
    core::fmt,
    super::{
        Reader,
        ReferenceTypeOpcode,
        SimpleName,
    },
};

/// # SuperName
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum SuperName {
    ReferenceTypeOpcode(ReferenceTypeOpcode),
    SimpleName(SimpleName),
}

impl fmt::Debug for SuperName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("SuperName");
        match self {
            Self::ReferenceTypeOpcode(reference_type_opcode) => debug_tuple.field(reference_type_opcode),
            Self::SimpleName(simple_name) => debug_tuple.field(simple_name),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for SuperName {
    fn from(aml: &[u8]) -> Self {
        if ReferenceTypeOpcode::matches(aml) {
            Self::ReferenceTypeOpcode(aml.into())
        } else if SimpleName::matches(aml) {
            Self::SimpleName(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for SuperName {
    fn length(&self) -> usize {
        match self {
            Self::ReferenceTypeOpcode(reference_type_opcode) => reference_type_opcode.length(),
            Self::SimpleName(simple_name) => simple_name.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        ReferenceTypeOpcode::matches(aml)
        || SimpleName::matches(aml)
    }
}

