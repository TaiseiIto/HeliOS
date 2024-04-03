use {
    alloc::vec::Vec,
    core::{
        fmt,
        mem,
    },
    super::system_description,
};

/// # XSDT
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.8 Extended System Description Table (XSDT)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
}

impl Table {
    pub fn entries<'a>(&'a self) -> Vec<system_description::Table<'a>> {
        let table: *const Self = self as *const Self;
        let table: usize = table as usize;
        let first_entry: usize = table + mem::size_of::<Self>();
        let first_entry: *const u64 = first_entry as *const u64;
        let entries: usize = (self.header.table_size() - mem::size_of::<Self>()) / mem::size_of::<u64>();
        (0..entries)
            .map(|index| {
                let entry: u64 = unsafe {
                    *first_entry.add(index)
                };
                let entry: usize = entry as usize;
                let header: *const system_description::Header = entry as *const system_description::Header;
                let header: &system_description::Header = unsafe {
                    &*header
                };
                header.into()
            })
            .collect()
    }

    pub fn is_correct(&self) -> bool {
        self.header.is_correct() && self
            .entries()
            .iter()
            .all(|entry| entry.is_correct())
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let entries: Vec<system_description::Table> = self.entries();
        formatter
            .debug_struct("Table")
            .field("header", &header)
            .field("entries", &entries)
            .finish()
    }
}

