pub mod table;

use core::{
    fmt,
    str,
};

/// # Firmware Basic Boot Performance Table
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.24.6 Firmware Basic Boot Performance Table
#[repr(packed)]
pub struct Table {
    signature: [u8; 4],
    length: u32,
}

impl Table {
    fn signature(&self) -> &str {
        let signature: &[u8] = self.signature.as_slice();
        str::from_utf8(signature).unwrap()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let signature: &str = self.signature();
        let length: u32 = self.length;
        formatter
            .debug_struct("Table")
            .field("signature", &signature)
            .field("length", &length)
            .finish()
    }
}

