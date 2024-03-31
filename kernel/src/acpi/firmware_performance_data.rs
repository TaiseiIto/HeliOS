mod firmware_basic_boot_performance;
mod other;
mod record;

use {
    alloc::vec::Vec,
    core::{
        fmt,
        mem,
        slice,
    },
    crate::{
        com2_print,
        com2_println,
    },
    super::system_description,
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
        let table: *const Self = unsafe {
            table.add(1)
        };
        let table: *const u8 = table as *const u8;
        let size: usize = self.header.table_size() - mem::size_of::<Self>();
        unsafe {
            slice::from_raw_parts(table, size)
        }
    }

    fn iter<'a>(&'a self) -> PerformanceRecords<'a> {
        self.into()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let performance_records: Vec<PerformanceRecord> = self
            .iter()
            .collect();
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
            com2_println!("PerformanceRecords.next()");
            com2_println!("performance_record = {:#x?}", performance_record);
            com2_println!("remaining_bytes = {:#x?}", remaining_bytes);
            self.bytes = remaining_bytes;
            performance_record
        })
    }
}

#[derive(Debug)]
enum PerformanceRecord<'a> {
    FirmwareBasicBootPerformanceTablePointer(&'a firmware_basic_boot_performance::table::pointer::Record),
    Other(&'a other::Record)
}

impl<'a> PerformanceRecord<'a> {
    fn scan(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        bytes
            .get(0)
            .zip(bytes.get(1))
            .map(|(record_type_low, record_type_high)| {
                let record_type = (*record_type_low as u16) + ((*record_type_high as u16) << u8::BITS);
                match record_type {
                    0x0000 => {
                        let firmware_basic_boot_performance_table_pointer: *const u8 = record_type_low as *const u8;
                        let firmware_basic_boot_performance_table_pointer: *const firmware_basic_boot_performance::table::pointer::Record = firmware_basic_boot_performance_table_pointer as *const firmware_basic_boot_performance::table::pointer::Record;
                        let firmware_basic_boot_performance_table_pointer: &firmware_basic_boot_performance::table::pointer::Record = unsafe {
                            &*firmware_basic_boot_performance_table_pointer
                        };
                        let firmware_basic_boot_performance_table_pointer = Self::FirmwareBasicBootPerformanceTablePointer(firmware_basic_boot_performance_table_pointer);
                        com2_println!("firmware_basic_boot_performance_table_pointer.size() = {:#x?}", firmware_basic_boot_performance_table_pointer.size());
                        let remaining_bytes: &[u8] = &bytes[firmware_basic_boot_performance_table_pointer.size()..];
                        (firmware_basic_boot_performance_table_pointer, remaining_bytes)
                    },
                    _ => {
                        let other: *const u8 = record_type_low as *const u8;
                        let other: *const other::Record = other as *const other::Record;
                        let other: &other::Record = unsafe {
                            &*other
                        };
                        let other = Self::Other(other);
                        com2_println!("other.size() = {:#x?}", other.size());
                        let remaining_bytes: &[u8] = &bytes[other.size()..];
                        (other, remaining_bytes)
                    },
                }
            })
    }

    fn size(&self) -> usize {
        match self {
            Self::FirmwareBasicBootPerformanceTablePointer(firmware_basic_boot_performance_table_pointer) => firmware_basic_boot_performance_table_pointer.length(),
            Self::Other(other) => other.length(),
        }
    }
}

