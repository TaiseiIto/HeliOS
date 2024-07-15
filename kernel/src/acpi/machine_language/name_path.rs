use {
    core::fmt,
    super::{
        NullName,
        NULL_NAME,
    },
};

/// # NamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum NamePath {
    NameSeg,
    DualNamePath,
    MultiNamePath,
    NullName(NullName),
}

impl NamePath {
    pub fn length(&self) -> usize {
        match self {
            Self::NameSeg => unimplemented!(),
            Self::DualNamePath => unimplemented!(),
            Self::MultiNamePath => unimplemented!(),
            Self::NullName(null_name) => null_name.length(),
        }
    }
}

impl fmt::Debug for NamePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameSeg => write!(formatter, "NamePath::NameSeg"),
            Self::DualNamePath => write!(formatter, "NamePath::DualNamePath"),
            Self::MultiNamePath => write!(formatter, "NamePath::MultiNamePath"),
            Self::NullName(null_name) => formatter
                .debug_tuple("NamePath")
                .field(null_name)
                .finish(),
        }
    }
}

impl From<&[u8]> for NamePath {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            NULL_NAME => {
                let null_name: NullName = aml.into();
                Self::NullName(null_name)
            },
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

