use {
    core::fmt,
    super::{
        Guid,
        Void,
    },
};

#[derive(Clone)]
#[repr(C)]
pub struct Tables<'a> {
    number_of_table_entries: usize,
    configuration_table: &'a Table,
}

impl Tables<'_> {
    pub fn iter(&self) -> impl Iterator<Item = &Table> {
        (0..self.number_of_table_entries)
            .map(|index| {
                let table: &Table = self.configuration_table;
                let table: *const Table = table as *const Table;
                unsafe {
                    &*table.add(index)
                }
            })
    }
}

impl fmt::Debug for Tables<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_list()
            .entries(self.iter())
            .finish()
    }
}

/// # EFI_CONFIGURATION_TABLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.6 EFI Configuration Table & Properties Table
#[derive(Debug)]
#[repr(C)]
pub struct Table {
    vendor_guid: Guid,
    vendor_table: *const Void,
}

