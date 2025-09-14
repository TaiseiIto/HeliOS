use {
    super::null,
    alloc::{string::String, vec::Vec},
    core::{fmt, iter, slice},
};

/// # CHAR16
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 2.3.1 Data Types
pub type Char16 = u16;

/// # Null terminated string
#[repr(C)]
pub struct NullTerminatedString<'a>(&'a Char16);

impl NullTerminatedString<'static> {
    pub fn null() -> Self {
        Self(null())
    }

    pub fn string2vec(string: &str) -> Vec<u16> {
        string
            .chars()
            .map(|character| character as u16)
            .chain(iter::once(0))
            .collect()
    }
}

impl fmt::Debug for NullTerminatedString<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string: String = self.into();
        string.fmt(formatter)
    }
}

impl<'a> From<&'a Char16> for NullTerminatedString<'a> {
    fn from(string: &'a Char16) -> Self {
        Self(string)
    }
}

impl<'a> From<&'a Vec<u16>> for NullTerminatedString<'a> {
    fn from(string: &'a Vec<u16>) -> Self {
        Self(&string[0])
    }
}

impl From<&NullTerminatedString<'_>> for String {
    fn from(string: &NullTerminatedString<'_>) -> Self {
        Self::from_utf16(string.into()).unwrap()
    }
}

impl<'a> From<&'a NullTerminatedString<'a>> for &'a [u16] {
    fn from(string: &'a NullTerminatedString<'a>) -> Self {
        let string: &u16 = string.0;
        let string: *const u16 = string as *const u16;
        let length: usize = (0..)
            .take_while(|index| {
                let index: usize = *index;
                let string: u16 = unsafe { string.add(index).read_volatile() };
                string != 0
            })
            .max()
            .map(|max_index| max_index + 1)
            .unwrap_or_default();
        unsafe { slice::from_raw_parts(string, length) }
    }
}
