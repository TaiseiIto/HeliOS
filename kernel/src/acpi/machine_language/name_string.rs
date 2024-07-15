use {
    alloc::vec::Vec,
    core::fmt,
    super::{
        NamePath,
        PREFIX_PATH,
        PrefixPath,
        ROOT_CHAR,
        RootChar,
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
    PrefixPathNamePath {
        prefix_path: Vec<PrefixPath>,
        name_path: NamePath,
    },
}

impl NameString {
    pub fn length(&self) -> usize {
        match self {
            Self::RootCharNamePath {
                root_char,
                name_path,
            } => root_char.length() + name_path.length(),
            Self::PrefixPathNamePath {
                prefix_path,
                name_path,
            } => prefix_path
                .iter()
                .map(|prefix_path| prefix_path.length())
                .sum::<usize>() + name_path.length(),
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
            Self::PrefixPathNamePath {
                prefix_path,
                name_path,
            } => formatter
                .debug_tuple("NameString")
                .field(prefix_path)
                .field(name_path)
                .finish(),
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
            _ => {
                let mut aml: &[u8] = aml;
                let mut prefix_path: Vec<PrefixPath> = Vec::new();
                while *aml.first().unwrap() == PREFIX_PATH {
                    let new_prefix_path: PrefixPath = aml.into();
                    aml = &aml[new_prefix_path.length()..];
                    prefix_path.push(new_prefix_path);
                }
                unimplemented!()
            }
        }
    }
}

