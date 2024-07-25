use {
    core::fmt,
    super::{
        DefWhile,
        Reader,
    },
};

/// # StatementOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
pub enum StatementOpcode {
    DefWhile(DefWhile),
}

impl fmt::Debug for StatementOpcode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("StatementOpcode");
        match self {
            Self::DefWhile(def_while) => debug_tuple.field(def_while),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for StatementOpcode {
    fn from(aml: &[u8]) -> Self {
        if DefWhile::matches(aml) {
            Self::DefWhile(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for StatementOpcode {
    fn length(&self) -> usize {
        match self {
            Self::DefWhile(def_while) => def_while.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        DefWhile::matches(aml)
    }
}

