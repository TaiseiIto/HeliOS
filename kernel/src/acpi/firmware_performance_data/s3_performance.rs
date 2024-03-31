pub mod table;

use core::{
    fmt,
    mem,
    slice,
    str,
};

/// # S3 Performance Table
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.24.8 S3 Performance Table
#[repr(packed)]
pub struct Table {
    signature: [u8; 4],
    length: u32,
}

impl Table {
    fn bytes(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let bytes: *const Self = unsafe {
            table.add(1)
        };
        let bytes: *const u8 = bytes as *const u8;
        let length: usize = self.length() - mem::size_of::<Self>();
        unsafe {
            slice::from_raw_parts(bytes, length)
        }
    }

    fn length(&self) -> usize {
        self.length as usize
    }

    fn signature(&self) -> &str {
        let signature: &[u8] = self.signature.as_slice();
        str::from_utf8(signature).unwrap()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let signature: &str = self.signature();
        let length: usize = self.length();
        let bytes: &[u8] = self.bytes();
        formatter
            .debug_struct("Table")
            .field("signature", &signature)
            .field("length", &length)
            .field("bytes", &bytes)
            .finish()
    }
}

