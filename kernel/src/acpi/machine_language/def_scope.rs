use core::fmt;

/// # DefScope
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 20.2.5.1 Namespace Modifier Objects Encoding
pub struct Symbol {
}

impl fmt::Debug for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "DefScope")
    }
}

impl From<&[u8]> for Symbol {
    fn from(bytes: &[u8]) -> Self {
        match bytes.first() {
            Some(first_byte) => match first_byte {
                0x10 => Self {},
                unknown_byte => panic!("Unknown byte {:#x?}", unknown_byte),
            }
            None => unimplemented!(),
        }
    }
}

