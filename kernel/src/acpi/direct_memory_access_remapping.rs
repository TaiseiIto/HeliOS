mod other;

use {
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
        slice,
    },
    super::system_description,
};

/// # DMA Remapping Table
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.1 DMA Remapping Reporting Structure
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    host_address_width: u8,
    flags: Flags,
    reserved0: [u8; 10],
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

    fn iter<'a>(&'a self) -> Structures<'a> {
        self.into()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let host_address_width: u8 = self.host_address_width;
        let flags: Flags = self.flags;
        formatter
            .debug_struct("Table")
            .field("header", &header)
            .field("host_address_width", &host_address_width)
            .field("flags", &flags)
            .finish()
    }
}

#[bitfield(u8)]
struct Flags {
    intr_remap: bool,
    x2apic_opt_out: bool,
    dma_ctrl_platform_opt_in_flag: bool,
    #[bits(5, access = RO)]
    reserved0: u8,
}

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

