use {
    alloc::{string::String, vec, vec::Vec},
    core::{fmt, str},
};

/// # VPD Format
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I.1. VPD Format. Figure I-5: VPD Format
pub struct Format {
    keyword: Keyword,
    length: u8,
    data: Vec<u8>,
}

impl Format {
    fn from_byte_iterator<T>(byte_iterator: &mut T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        let keyword_low: Option<u8> = byte_iterator.next();
        let keyword_high: Option<u8> = byte_iterator.next();
        let length: Option<u8> = byte_iterator.next();
        match (keyword_low, keyword_high, length) {
            (Some(keyword_low), Some(keyword_high), Some(length)) => {
                let keyword: Vec<u8> = vec![keyword_low, keyword_high];
                let keyword: Keyword = str::from_utf8(&keyword).unwrap().into();
                let data: Vec<u8> = (0..length).map(|_| byte_iterator.next().unwrap()).collect();
                Some(Self {
                    keyword,
                    length,
                    data,
                })
            }
            _ => None,
        }
    }

    fn string(&self) -> Option<String> {
        let Self {
            keyword,
            length: _,
            data,
        } = self;
        match keyword {
            Keyword::Cp
            | Keyword::Ec
            | Keyword::Fg
            | Keyword::Lc
            | Keyword::Mn
            | Keyword::Pg
            | Keyword::Pn
            | Keyword::Sn
            | Keyword::V(_)
            | Keyword::Ya
            | Keyword::Y(_) => String::from_utf8(data.to_vec()).ok(),
            Keyword::Rv | Keyword::Rw => None,
        }
    }
}

impl fmt::Debug for Format {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct: fmt::DebugStruct = formatter.debug_struct("Format");
        debug_struct
            .field("keyword", &self.keyword)
            .field("length", &self.length);
        if let Some(string) = self.string() {
            debug_struct.field("data", &string);
        }
        debug_struct.finish()
    }
}

pub struct FormatIterator<T>
where
    T: Iterator<Item = u8>,
{
    byte_iterator: T,
}

impl<T> FormatIterator<T>
where
    T: Iterator<Item = u8>,
{
    pub fn new(byte_iterator: T) -> Self {
        Self { byte_iterator }
    }
}

impl<T> Iterator for FormatIterator<T>
where
    T: Iterator<Item = u8>,
{
    type Item = Format;

    fn next(&mut self) -> Option<Self::Item> {
        let Self { byte_iterator } = self;
        Self::Item::from_byte_iterator(byte_iterator)
    }
}

/// # VPD Keyword
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I.3.1.1. Read-Only Fields
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I.3.1.2. Read/Write Fields
pub enum Keyword {
    Cp,
    Ec,
    Fg,
    Lc,
    Mn,
    Pg,
    Pn,
    Rv,
    Rw,
    Sn,
    V(char),
    Ya,
    Y(char),
}

impl From<&str> for Keyword {
    fn from(keyword: &str) -> Self {
        match keyword {
            "CP" => Self::Cp,
            "EC" => Self::Ec,
            "FG" => Self::Fg,
            "LC" => Self::Lc,
            "MN" => Self::Mn,
            "PG" => Self::Pg,
            "PN" => Self::Pn,
            "RV" => Self::Rv,
            "RW" => Self::Rw,
            "SN" => Self::Sn,
            "YA" => Self::Ya,
            keyword => {
                let keyword: Vec<char> = keyword.chars().collect();
                match keyword.as_slice().chunks(2).next().unwrap() {
                    ['V', x] => {
                        assert!(matches!(x, '0'..='9' | 'A'..= 'Z'));
                        Self::V(*x)
                    }
                    ['Y', x] => {
                        assert!(matches!(x, '0'..='9' | 'B'..= 'Z'));
                        Self::Y(*x)
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

impl fmt::Debug for Keyword {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cp => write!(formatter, "CP"),
            Self::Ec => write!(formatter, "EC"),
            Self::Fg => write!(formatter, "FG"),
            Self::Lc => write!(formatter, "LC"),
            Self::Mn => write!(formatter, "MN"),
            Self::Pg => write!(formatter, "PG"),
            Self::Pn => write!(formatter, "PN"),
            Self::Rv => write!(formatter, "RV"),
            Self::Rw => write!(formatter, "RW"),
            Self::Sn => write!(formatter, "SN"),
            Self::V(x) => write!(formatter, "V{}", x),
            Self::Ya => write!(formatter, "YA"),
            Self::Y(x) => write!(formatter, "Y{}", x),
        }
    }
}
