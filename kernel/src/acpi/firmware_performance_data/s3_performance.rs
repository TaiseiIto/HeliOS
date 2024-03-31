pub mod table;

use {
    alloc::vec::Vec,
    core::{
        fmt,
        mem,
        slice,
        str,
    },
    super::other,
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

    fn iter<'a>(&'a self) -> PerformanceRecords<'a> {
        self.into()
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
        let performance_records: Vec<PerformanceRecord> = self
            .iter()
            .collect();
        formatter
            .debug_struct("Table")
            .field("signature", &signature)
            .field("length", &length)
            .field("performance_records", &performance_records)
            .finish()
    }
}

struct PerformanceRecords<'a> {
    bytes: &'a [u8],
}

impl<'a> From<&'a Table> for PerformanceRecords<'a> {
    fn from(table: &'a Table) -> Self {
        let bytes: &[u8] = table.bytes();
        Self {
            bytes,
        }
    }
}

impl<'a> Iterator for PerformanceRecords<'a> {
    type Item = PerformanceRecord<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes: &[u8] = self.bytes;
        Self::Item::scan(bytes).map(|(performance_record, remaining_bytes)| {
            self.bytes = remaining_bytes;
            performance_record
        })
    }
}

#[derive(Debug)]
enum PerformanceRecord<'a> {
    Other(&'a other::Record),
}

impl<'a> PerformanceRecord<'a> {
    fn scan(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        bytes
            .get(0)
            .zip(bytes.get(1))
            .map(|(record_type_low, record_type_high)| {
                let record_type = (*record_type_low as u16) + ((*record_type_high as u16) << u8::BITS);
                match record_type {
                    _ => {
                        let other: *const u8 = record_type_low as *const u8;
                        let other: *const other::Record = other as *const other::Record;
                        let other: &other::Record = unsafe {
                            &*other
                        };
                        let other = Self::Other(other);
                        let remaining_bytes: &[u8] = &bytes[other.size()..];
                        (other, remaining_bytes)
                    },
                }
            })
    }

    fn size(&self) -> usize {
        match self {
            Self::Other(other) => other.length(),
        }
    }
}

