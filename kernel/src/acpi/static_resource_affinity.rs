use {
    core::{
        fmt,
        mem,
        slice,
    },
    super::system_description,
};

/// # System Resource Affinity Table (SRAT)
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16 System Resource Affinity Table (SRAT)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    reserved0: [u8; 12],
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn bytes(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let table: usize = table as usize;
        let first_byte: usize = table + mem::size_of::<Self>();
        let first_byte: *const u8 = first_byte as *const u8;
        let size: usize = self.header.table_size() - mem::size_of::<Self>();
        unsafe {
            slice::from_raw_parts(first_byte, size)
        }
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let structures: Structures = self.into();
        formatter
            .debug_struct("Table")
            .field("header", &header)
            .field("structures", &structures)
            .finish()
    }
}

#[derive(Debug)]
struct Structures<'a> {
    bytes: &'a [u8],
}

impl<'a> From<&'a Table> for Structures<'a> {
    fn from(table: &'a Table) -> Self {
        let bytes: &[u8] = table.bytes();
        Self {
            bytes,
        }
    }
}

