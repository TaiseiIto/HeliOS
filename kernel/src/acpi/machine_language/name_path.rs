use {
    alloc::vec::Vec,
    core::fmt,
    super::{
        LeadNameChar,
        NameChar,
        NullName,
        NULL_NAME,
    },
};

/// # NamePath
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub enum NamePath {
    NameSeg {
        lead_name_char: LeadNameChar,
        name_char: [NameChar; 3],
    },
    DualNamePath,
    MultiNamePath,
    NullName(NullName),
}

impl NamePath {
    pub fn length(&self) -> usize {
        match self {
            Self::NameSeg {
                lead_name_char,
                name_char,
            } => lead_name_char.length() + name_char
                .as_slice()
                .iter()
                .map(|name_char| name_char.length())
                .sum::<usize>(),
            Self::DualNamePath => unimplemented!(),
            Self::MultiNamePath => unimplemented!(),
            Self::NullName(null_name) => null_name.length(),
        }
    }
}

impl fmt::Debug for NamePath {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NameSeg {
                lead_name_char,
                name_char,
            } => formatter
                .debug_tuple("NamePath")
                .field(lead_name_char)
                .field(name_char)
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

impl From<&[u8]> for NamePath {
    fn from(aml: &[u8]) -> Self {
        match *aml.first().unwrap() {
            NULL_NAME => {
                let null_name: NullName = aml.into();
                Self::NullName(null_name)
            },
            first_byte => {
                let lead_name_char: LeadNameChar = aml.into();
                let aml: &[u8] = &aml[lead_name_char.length()..];
                let (aml, name_char): (&[u8], Vec<NameChar>) = (0..3)
                    .fold((aml, Vec::new()), |(aml, mut name_char), _| {
                        let new_name_char: NameChar = aml.into();
                        let aml: &[u8] = &aml[new_name_char.length()..];
                        name_char.push(new_name_char);
                        (aml, name_char)
                    });
                let name_char: [NameChar; 3] = name_char
                    .try_into()
                    .unwrap();
                Self::NameSeg {
                    lead_name_char,
                    name_char,
                }
            },
        }
    }
}

