use {
    alloc::vec::Vec,
    core::{
        fmt,
        iter,
    },
};

/// # CHAR16
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 2.3.1 Data Types
pub type Char16 = u16;

/// # Null terminated string
#[derive(Clone)]
#[repr(C)]
pub struct NullTerminatedString<'a>(&'a Char16);

impl fmt::Debug for NullTerminatedString<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        iter::once('"')
            .chain(self.clone())
            .chain(iter::once('"'))
            .fold(Ok(()), |result, character| result.and(write!(formatter, "{}", character)))
    }
}

impl<'a> From<&'a Vec<u16>> for NullTerminatedString<'a> {
    fn from(string: &'a Vec<u16>) -> Self {
        Self(&string[0])
    }
}

impl Iterator for NullTerminatedString<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let output = *self.0;
        match output {
            0 => None,
            output => {
                let output = Self::Item::from_u32(output as u32);
                if output.is_some() {
                    self.0 = unsafe {
                        &*(self.0 as *const Char16).add(1)
                    };
                }
                output
            },
        }
    }
}

