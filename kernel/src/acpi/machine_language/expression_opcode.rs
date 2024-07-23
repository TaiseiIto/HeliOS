use {
    core::fmt,
    super::{
        DefLLess,
        DefSizeOf,
        DefStore,
        DefSubtract,
        DefToBuffer,
        DefToHexString,
        Reader,
    },
};

/// # ExpressionOpcode
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.4 Expression Opcodes Encoding
pub enum ExpressionOpcode {
    DefLLess(DefLLess),
    DefSizeOf(DefSizeOf),
    DefStore(DefStore),
    DefSubtract(DefSubtract),
    DefToBuffer(DefToBuffer),
    DefToHexString(DefToHexString),
}

impl fmt::Debug for ExpressionOpcode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("ExpressionOpcode");
        match self {
            Self::DefLLess(def_l_less) => debug_tuple.field(def_l_less),
            Self::DefSizeOf(def_size_of) => debug_tuple.field(def_size_of),
            Self::DefStore(def_store) => debug_tuple.field(def_store),
            Self::DefSubtract(def_subtract) => debug_tuple.field(def_subtract),
            Self::DefToBuffer(def_to_buffer) => debug_tuple.field(def_to_buffer),
            Self::DefToHexString(def_to_hex_string) => debug_tuple.field(def_to_hex_string),
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for ExpressionOpcode {
    fn from(aml: &[u8]) -> Self {
        if DefLLess::matches(aml) {
            Self::DefLLess(aml.into())
        } else if DefSizeOf::matches(aml) {
            Self::DefSizeOf(aml.into())
        } else if DefStore::matches(aml) {
            Self::DefStore(aml.into())
        } else if DefSubtract::matches(aml) {
            Self::DefSubtract(aml.into())
        } else if DefToBuffer::matches(aml) {
            Self::DefToBuffer(aml.into())
        } else if DefToHexString::matches(aml) {
            Self::DefToHexString(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for ExpressionOpcode {
    fn length(&self) -> usize {
        match self {
            Self::DefLLess(def_l_less) => def_l_less.length(),
            Self::DefSizeOf(def_size_of) => def_size_of.length(),
            Self::DefStore(def_store) => def_store.length(),
            Self::DefSubtract(def_subtract) => def_subtract.length(),
            Self::DefToBuffer(def_to_buffer) => def_to_buffer.length(),
            Self::DefToHexString(def_to_hex_string) => def_to_hex_string.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        DefLLess::matches(aml)
        || DefSizeOf::matches(aml)
        || DefStore::matches(aml)
        || DefSubtract::matches(aml)
        || DefToBuffer::matches(aml)
        || DefToHexString::matches(aml)
    }
}

