use core::fmt;

/// # TermObj
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5 Term Objects Encoding
pub enum Symbol<'a> {
    Object {
        bytes: &'a [u8],
    },
}

impl fmt::Debug for Symbol<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "TermObj")
    }
}

impl<'a> From<&'a [u8]> for Symbol<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self::Object {
            bytes
        }
    }
}

