use core::fmt;

/// # CHAR16
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.3 EFI System Table
pub struct Char16(u16);

/// # Null terminated string
#[derive(Clone)]
pub struct NullTerminatedString<'a>(&'a Char16);

impl fmt::Debug for NullTerminatedString<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.clone().fold(Ok(()), |result, character| result.and(write!(formatter, "{}", character)))
    }
}

impl Iterator for NullTerminatedString<'_> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let output = (self.0).0;
        match output {
            0 => None,
            output => {
                let output = Self::Item::from_u32(output as u32);
                if let Some(_) = output {
                    self.0 = unsafe {
                        &*(self.0 as *const Char16).add(1)
                    };
                }
                output
            },
        }
    }
}

