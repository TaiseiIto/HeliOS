use {
    alloc::{
        string::String,
        vec::Vec,
    },
    core::{
        fmt,
        iter,
    },
    super::{
        LeadNameChar,
        NameChar,
        Reader,
    },
};

/// # NameSeg
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.2 Name Objects Encoding
pub struct NameSeg {
    lead_name_char: LeadNameChar,
    name_char: [NameChar; 3],
}

impl fmt::Debug for NameSeg {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            lead_name_char,
            name_char,
        } = self;
        formatter
            .debug_tuple("NameSeg")
            .field(lead_name_char)
            .field(name_char)
            .finish()
    }
}

impl From<&NameSeg> for String {
    fn from(name_seg: &NameSeg) -> Self {
        let NameSeg {
            lead_name_char,
            name_char,
        } = name_seg;
        iter::once({
            let lead_name_char: char = lead_name_char.into();
            lead_name_char
        })
        .chain(name_char
            .as_slice()
            .iter()
            .map(|name_char| name_char.into()))
        .collect()
    }
}

impl From<&[u8]> for NameSeg {
    fn from(aml: &[u8]) -> Self {
        let (lead_name_char, aml): (LeadNameChar, &[u8]) = LeadNameChar::read(aml);
        let (aml, name_char): (&[u8], Vec<NameChar>) = (0..3)
            .fold((aml, Vec::new()), |(aml, mut name_char), _| {
                let (new_name_char, aml): (NameChar, &[u8]) = NameChar::read(aml);
                name_char.push(new_name_char);
                (aml, name_char)
            });
        let name_char: [NameChar; 3] = name_char
            .try_into()
            .unwrap();
        Self {
            lead_name_char,
            name_char,
        }
    }
}

impl Reader<'_> for NameSeg {
    fn length(&self) -> usize {
        let Self {
            lead_name_char,
            name_char,
        } = self;
        lead_name_char.length() + name_char
            .as_slice()
            .iter()
            .map(|name_char| name_char.length())
            .sum::<usize>()
    }

    fn matches(aml: &[u8]) -> bool {
        true
    }
}

