mod firmware_basic_boot_performance;
mod other;
mod record;
mod s3_performance;

use {
    super::system_description,
    alloc::vec::Vec,
    core::{fmt, mem::size_of, slice},
};

/// # Firmware Performance Data Table (FPDT)
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.24 Firmware Performance Data Table (FPDT)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn bytes(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let table: *const Self = unsafe { table.add(1) };
        let table: *const u8 = table as *const u8;
        let size: usize = self.header.table_size() - size_of::<Self>();
        unsafe { slice::from_raw_parts(table, size) }
    }

    fn iter(&self) -> PerformanceRecords<'_> {
        self.into()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let performance_records: Vec<PerformanceRecord> = self.iter().collect();
        formatter
            .debug_struct("Table")
            .field("header", &header)
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
        Self { bytes }
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
    FirmwareBasicBootPerformanceTablePointer(
        &'a firmware_basic_boot_performance::table::pointer::Record<'a>,
    ),
    Other(&'a other::Record),
    S3PerformanceTablePointer(&'a s3_performance::table::pointer::Record<'a>),
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
                        let record: *const firmware_basic_boot_performance::table::pointer::Record = record as *const firmware_basic_boot_performance::table::pointer::Record;
                        let record: &firmware_basic_boot_performance::table::pointer::Record = unsafe {
                            &*record
                        };
                        let record = Self::FirmwareBasicBootPerformanceTablePointer(record);
                        let remaining_bytes: &[u8] = &bytes[record.size()..];
                        (record, remaining_bytes)
                    },
                    0x0001 => {
                        let record: *const u8 = record_type_low as *const u8;
                        let record: *const s3_performance::table::pointer::Record = record as *const s3_performance::table::pointer::Record;
                        let record: &s3_performance::table::pointer::Record = unsafe {
                            &*record
                        };
                        let record = Self::S3PerformanceTablePointer(record);
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
            Self::FirmwareBasicBootPerformanceTablePointer(record) => record.length(),
            Self::Other(record) => record.length(),
            Self::S3PerformanceTablePointer(record) => record.length(),
        }
    }
}
