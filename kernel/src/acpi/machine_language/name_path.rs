use {
    core::fmt,
    super::NullName,
};

/// # NamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum NamePath {
    NameSeg,
    DualNamePath,
    MultiNamePath,
    NullName {
        null_name: NullName,
    },
}

impl fmt::Debug for NamePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameSeg => write!(formatter, "NamePath::NameSeg"),
            Self::DualNamePath => write!(formatter, "NamePath::DualNamePath"),
            Self::MultiNamePath => write!(formatter, "NamePath::MultiNamePath"),
            Self::NullName {
                null_name,
            } => formatter
                .debug_struct("NamePath")
                .field("null_name", null_name)
                .finish(),
        }
    }
}

impl From<&[u8]> for NamePath {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first().unwrap() {
            0x00 => {
                let null_name: NullName = bytes.into();
                Self::NullName {
                    null_name,
                }
            },
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

