use {
    alloc::string::String,
    core::fmt,
    super::{
        NameSeg,
        NullName,
        Reader,
    },
};

/// # NamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum NamePath {
    NameSeg(NameSeg),
    NullName(NullName),
}

impl fmt::Debug for NamePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("NameSeg");
        match self {
            Self::NameSeg(name_seg) => debug_tuple.field(name_seg),
            Self::NullName(null_name) => debug_tuple.field(null_name),
        };
        debug_tuple.finish()
    }
}

impl From<&NamePath> for String {
    fn from(name_path: &NamePath) -> Self {
        match name_path {
            NamePath::NameSeg(name_seg) => name_seg.into(),
            NamePath::NullName(null_name) => Self::new(),
        }
    }
}

impl From<&[u8]> for NamePath {
    fn from(aml: &[u8]) -> Self {
        if NameSeg::matches(aml) {
            Self::NameSeg(aml.into())
        } else if NullName::matches(aml) {
            Self::NullName(aml.into())
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for NamePath {
    fn length(&self) -> usize {
        match self {
            Self::NameSeg(name_seg) => name_seg.length(),
            Self::NullName(null_name) => null_name.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        NameSeg::matches(aml) || NullName::matches(aml)
    }
}

