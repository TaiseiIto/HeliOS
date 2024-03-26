mod other;

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

impl<'a> Iterator for Structures<'a> {
    type Item = Structure<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes: &[u8] = self.bytes;
        Self::Item::scan(bytes).map(|(structure, remaining_bytes)| {
            self.bytes = remaining_bytes;
            structure
        })
    }
}

#[derive(Debug)]
enum Structure<'a> {
    Other(&'a other::Structure),
}

impl<'a> Structure<'a> {
    fn scan(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        bytes
            .first()
            .map(|structure_type| {
                let other: *const u8 = structure_type as *const u8;
                let other: *const other::Structure = other as *const other::Structure;
                let other: &other::Structure = unsafe {
                    &*other
                };
                let other = Self::Other(other);
                let remaining_bytes: &[u8] = &bytes[other.size()..];
                (other, remaining_bytes)
            })
    }

    fn size(&self) -> usize {
        match self {
            Self::Other(other) => other.length(),
        }
    }
}

