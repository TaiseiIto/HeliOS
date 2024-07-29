use {
    core::fmt,
    super::{
        DefIfElse,
        DefReturn,
        DefWhile,
        Reader,
    },
};

/// # StatementOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.3 Statement Opcodes Encoding
pub enum StatementOpcode {
    DefIfElse(DefIfElse),
    DefReturn(DefReturn),
    DefWhile(DefWhile),
}

impl fmt::Debug for StatementOpcode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("StatementOpcode");
        match self {
            Self::DefIfElse(def_if_else) => debug_tuple.field(def_if_else),
            Self::DefReturn(def_return) => debug_tuple.field(def_return),
            Self::DefWhile(def_while) => debug_tuple.field(def_while),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for StatementOpcode {
    fn from(aml: &[u8]) -> Self {
        if DefIfElse::matches(aml) {
            Self::DefIfElse(aml.into())
        } else if DefReturn::matches(aml) {
            Self::DefReturn(aml.into())
        } else if DefWhile::matches(aml) {
            Self::DefWhile(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for StatementOpcode {
    fn length(&self) -> usize {
        match self {
            Self::DefIfElse(def_if_else) => def_if_else.length(),
            Self::DefReturn(def_return) => def_return.length(),
            Self::DefWhile(def_while) => def_while.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        DefIfElse::matches(aml)
        || DefReturn::matches(aml)
        || DefWhile::matches(aml)
    }
}

