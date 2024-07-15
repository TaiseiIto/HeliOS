use {
    core::fmt,
    super::{
        NamePath,
        RootChar,
        ROOT_CHAR,
    },
};

/// # NameString
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum NameString {
    RootCharNamePath {
        root_char: RootChar,
        name_path: NamePath,
    },
    PrefixPathNamePath,
}

impl NameString {
    pub fn length(&self) -> usize {
        match self {
            Self::RootCharNamePath {
                root_char,
                name_path,
            } => root_char.length() + name_path.length(),
            Self::PrefixPathNamePath => unimplemented!(),
        }
    }
}

impl fmt::Debug for NameString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootCharNamePath {
                root_char,
                name_path,
            } => formatter
                .debug_tuple("NameString")
                .field(root_char)
                .field(name_path)
                .finish(),
            Self::PrefixPathNamePath => write!(formatter, "NameString::PrefixPathNamePath"),
        }
    }
}

impl From<&[u8]> for NameString {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            ROOT_CHAR => {
                let root_char: RootChar = aml.into();
                let aml: &[u8] = &aml[root_char.length()..];
                let name_path: NamePath = aml.into();
                Self::RootCharNamePath {
                    root_char,
                    name_path,
                }
            },
            unknown_byte => panic!("unknown_byte = {:#x?}", unknown_byte),
        }
    }
}

