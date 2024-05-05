pub mod resume_performance;
pub mod suspend_performance;
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

    fn iter(&self) -> PerformanceRecords<'_> {
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
    ResumePerformance(&'a resume_performance::Record),
    SuspendPerformance(&'a suspend_performance::Record),
}

impl<'a> PerformanceRecord<'a> {
    fn scan(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        bytes
            .first()
            .zip(bytes.get(1))
            .map(|(record_type_low, record_type_high)| {
                let record_type = (*record_type_low as u16) + ((*record_type_high as u16) << u8::BITS);
                match record_type {
                    0x0000 => {
                        let record: *const u8 = record_type_low as *const u8;
                        let record: *const resume_performance::Record = record as *const resume_performance::Record;
                        let record: &resume_performance::Record = unsafe {
                            &*record
                        };
                        let record = Self::ResumePerformance(record);
                        let remaining_bytes: &[u8] = &bytes[record.size()..];
                        (record, remaining_bytes)
                    },
                    0x0001 => {
                        let record: *const u8 = record_type_low as *const u8;
                        let record: *const suspend_performance::Record = record as *const suspend_performance::Record;
                        let record: &suspend_performance::Record = unsafe {
                            &*record
                        };
                        let record = Self::SuspendPerformance(record);
                        let remaining_bytes: &[u8] = &bytes[record.size()..];
                        (record, remaining_bytes)
                    },
                    _ => {
                        let record: *const u8 = record_type_low as *const u8;
                        let record: *const other::Record = record as *const other::Record;
                        let record: &other::Record = unsafe {
                            &*record
                        };
                        let record = Self::Other(record);
                        let remaining_bytes: &[u8] = &bytes[record.size()..];
                        (record, remaining_bytes)
                    },
                }
            })
    }

    fn size(&self) -> usize {
        match self {
            Self::Other(record) => record.length(),
            Self::ResumePerformance(record) => record.length(),
            Self::SuspendPerformance(record) => record.length(),
        }
    }
}

