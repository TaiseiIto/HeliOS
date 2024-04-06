use {
    core::fmt,
    crate::acpi,
    super::{
        Guid,
        Void,
    },
};

#[repr(C)]
pub struct Tables<'a> {
    number_of_table_entries: usize,
    configuration_table: &'a mut Table,
}

impl Tables<'_> {
    pub fn rsdp_mut(&mut self) -> &mut acpi::root_system_description::Pointer {
        let acpi_table_guid = Guid::new(0x8868e871, 0xe4f1, 0x11d3, [0xbc, 0x22, 0x0, 0x80, 0xc7, 0x3c, 0x88, 0x81]);
        let rsdp: *mut acpi::root_system_description::Pointer = self.iter_mut()
            .find(|table| table.vendor_guid == acpi_table_guid)
            .unwrap()
            .vendor_table as *mut acpi::root_system_description::Pointer;
        unsafe {
            &mut *rsdp
        }
    }

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

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Table> {
        (0..self.number_of_table_entries)
            .map(|index| {
                let table: &mut Table = self.configuration_table;
                let table: *mut Table = table as *mut Table;
                unsafe {
                    &mut *table.add(index)
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

