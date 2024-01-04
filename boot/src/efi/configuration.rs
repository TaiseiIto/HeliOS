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

impl fmt::Debug for Tables<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_list()
            .entries(self.clone())
            .finish()
    }
}

impl<'a> Iterator for Tables<'a> {
    type Item = &'a Table;

    fn next(&mut self) -> Option<Self::Item> {
        match self.number_of_table_entries {
            0 => None,
            _ => {
                let output: &Table = self.configuration_table;
                let configuration_table: &Table = self.configuration_table;
                let configuration_table: *const Table = configuration_table as *const Table;
                self.number_of_table_entries -= 1;
                self.configuration_table = unsafe {
                    &*configuration_table.add(1)
                };
                Some(output)
            },
        }
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

