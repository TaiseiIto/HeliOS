use {
    alloc::vec::Vec,
    core::fmt,
    super::{
        NamePath,
        PrefixPath,
        Reader,
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

impl fmt::Debug for NameString {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_tuple: fmt::DebugTuple = formatter.debug_tuple("NameString");
        match self {
            Self::RootCharNamePath {
                root_char,
                name_path,
            } => debug_tuple
                .field(root_char)
                .field(name_path),
            Self::PrefixPathNamePath {
                prefix_path,
                name_path,
            } => {
                if !prefix_path.is_empty() {
                    debug_tuple.field(prefix_path);
                }
                debug_tuple.field(name_path)
            },
        };
        debug_tuple.finish()
    }
}

impl From<&[u8]> for NameString {
    fn from(aml: &[u8]) -> Self {
        if RootChar::matches(aml) {
            let (root_char, aml): (RootChar, &[u8]) = RootChar::read(aml);
            let name_path: NamePath = aml.into();
            Self::RootCharNamePath {
                root_char,
                name_path,
            }
        } else if PrefixPath::matches(aml) || NamePath::matches(aml) {
            let mut aml: &[u8] = aml;
            let mut prefix_path: Vec<PrefixPath> = Vec::new();
            while PrefixPath::matches(aml) {
                let (new_prefix_path, remaining_aml): (PrefixPath, &[u8]) = PrefixPath::read(aml);
                aml = remaining_aml;
                prefix_path.push(new_prefix_path);
            }
            let name_path: NamePath = aml.into();
            Self::PrefixPathNamePath {
                prefix_path,
                name_path,
            }
        } else {
            panic!("aml = {:#x?}", aml)
        }
    }
}

impl Reader<'_> for NameString {
    fn length(&self) -> usize {
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

    fn matches(aml: &[u8]) -> bool {
        RootChar::matches(aml)
        || PrefixPath::matches(aml)
        || NamePath::matches(aml)
    }

    fn read(aml: &[u8]) -> (Self, &[u8]) {
        let symbol: Self = aml.into();
        let aml: &[u8] = &aml[symbol.length()..];
        (symbol, aml)
    }
}

