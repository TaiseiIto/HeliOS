use {
    alloc::string::String,
    core::fmt,
    super::{
        DUAL_NAME_PREFIX,
        MULTI_NAME_PREFIX,
        NULL_NAME,
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
    DualNamePath,
    MultiNamePath,
    NullName(NullName),
}

impl fmt::Debug for NamePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameSeg(name_seg) => formatter
                .debug_tuple("NameSeg")
                .field(name_seg)
                .finish(),
            Self::DualNamePath => write!(formatter, "NamePath::DualNamePath"),
            Self::MultiNamePath => write!(formatter, "NamePath::MultiNamePath"),
            Self::NullName(null_name) => formatter
                .debug_tuple("NamePath")
                .field(null_name)
                .finish(),
        }
    }
}

impl From<&NamePath> for String {
    fn from(name_path: &NamePath) -> Self {
        match name_path {
            NamePath::NameSeg(name_seg) => name_seg.into(),
            NamePath::DualNamePath => unimplemented!(),
            NamePath::MultiNamePath => unimplemented!(),
            NamePath::NullName(null_name) => Self::new(),
        }
    }
}

impl From<&[u8]> for NamePath {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            DUAL_NAME_PREFIX => unimplemented!(),
            MULTI_NAME_PREFIX => unimplemented!(),
            NULL_NAME => Self::NullName(aml.into()),
            _ => Self::NameSeg(aml.into()),
        }
    }
}

impl Reader<'_> for NamePath {
    fn length(&self) -> usize {
        match self {
            Self::NameSeg(name_seg) => name_seg.length(),
            Self::DualNamePath => unimplemented!(),
            Self::MultiNamePath => unimplemented!(),
            Self::NullName(null_name) => null_name.length(),
        }
    }

    fn matches(aml: &[u8]) -> bool {
        NameSeg::matches(aml) || NullName::matches(aml)
    }
}

