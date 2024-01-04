use {
    core::fmt,
    super::{
        Guid,
        Void,
    },
};

#[repr(C)]
pub struct Tables<'a> {
    number_of_table_entries: usize,
    configuration_table: &'a Table,
}

impl fmt::Debug for Tables<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_list()
            .entries((0..self.number_of_table_entries)
                .map(|i| {
                    let first_table: &Table = self.configuration_table;
                    let first_table: *const Table = first_table as *const Table;
                    unsafe {
                        &*first_table.add(i)
                    }
                }))
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

