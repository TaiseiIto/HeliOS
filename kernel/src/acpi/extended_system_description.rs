use {
    super::{
        fixed_acpi_description, high_precision_event_timer, multiple_apic_description,
        secondary_system_description, system_description,
    },
    alloc::vec::Vec,
    core::{fmt, mem, slice},
};

/// # XSDT
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.8 Extended System Description Table (XSDT)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
}

impl Table {
    pub fn entries(&self) -> Vec<system_description::Table<'_>> {
        self.bytes()
            .chunks(mem::size_of::<usize>())
            .map(|entry_address_bytes| {
                let entry: usize = entry_address_bytes
                    .iter()
                    .rev()
                    .fold(0usize, |entry_address, byte| {
                        (entry_address << u8::BITS) + (*byte as usize)
                    });
                let header: *const system_description::Header =
                    entry as *const system_description::Header;
                let header: &system_description::Header = unsafe { &*header };
                header.into()
            })
            .collect()
    }

    pub fn fadt(&self) -> &fixed_acpi_description::Table {
        self.bytes()
            .chunks(mem::size_of::<usize>())
            .find_map(|entry_address_bytes| {
                let entry: usize = entry_address_bytes
                    .iter()
                    .rev()
                    .fold(0usize, |entry_address, byte| {
                        (entry_address << u8::BITS) + (*byte as usize)
                    });
                let header: *const system_description::Header =
                    entry as *const system_description::Header;
                let header: &system_description::Header = unsafe { &*header };
                (header.signature() == "FACP").then(|| {
                    let table: *const fixed_acpi_description::Table =
                        entry as *const fixed_acpi_description::Table;
                    unsafe { &*table }
                })
            })
            .unwrap()
    }

    pub fn fadt_mut(&mut self) -> &mut fixed_acpi_description::Table {
        self.bytes()
            .chunks(mem::size_of::<usize>())
            .find_map(|entry_address_bytes| {
                let entry: usize = entry_address_bytes
                    .iter()
                    .rev()
                    .fold(0usize, |entry_address, byte| {
                        (entry_address << u8::BITS) + (*byte as usize)
                    });
                let header: *const system_description::Header =
                    entry as *const system_description::Header;
                let header: &system_description::Header = unsafe { &*header };
                (header.signature() == "FACP").then(|| {
                    let table: *mut fixed_acpi_description::Table =
                        entry as *mut fixed_acpi_description::Table;
                    unsafe { &mut *table }
                })
            })
            .unwrap()
    }

    pub fn hpet(&self) -> &high_precision_event_timer::Table {
        self.bytes()
            .chunks(mem::size_of::<usize>())
            .find_map(|entry_address_bytes| {
                let entry: usize = entry_address_bytes
                    .iter()
                    .rev()
                    .fold(0usize, |entry_address, byte| {
                        (entry_address << u8::BITS) + (*byte as usize)
                    });
                let header: *const system_description::Header =
                    entry as *const system_description::Header;
                let header: &system_description::Header = unsafe { &*header };
                (header.signature() == "HPET").then(|| {
                    let table: *const high_precision_event_timer::Table =
                        entry as *const high_precision_event_timer::Table;
                    unsafe { &*table }
                })
            })
            .unwrap()
    }

    pub fn hpet_mut(&mut self) -> &mut high_precision_event_timer::Table {
        self.bytes_mut()
            .chunks(mem::size_of::<usize>())
            .find_map(|entry_address_bytes| {
                let entry: usize = entry_address_bytes
                    .iter()
                    .rev()
                    .fold(0usize, |entry_address, byte| {
                        (entry_address << u8::BITS) + (*byte as usize)
                    });
                let header: *const system_description::Header =
                    entry as *const system_description::Header;
                let header: &system_description::Header = unsafe { &*header };
                (header.signature() == "HPET").then(|| {
                    let table: *mut high_precision_event_timer::Table =
                        entry as *mut high_precision_event_timer::Table;
                    unsafe { &mut *table }
                })
            })
            .unwrap()
    }

    pub fn is_correct(&self) -> bool {
        self.header.is_correct() && self.entries().iter().all(|entry| entry.is_correct())
    }

    pub fn madt(&self) -> &multiple_apic_description::Table {
        self.bytes()
            .chunks(mem::size_of::<usize>())
            .find_map(|entry_address_bytes| {
                let entry: usize = entry_address_bytes
                    .iter()
                    .rev()
                    .fold(0usize, |entry_address, byte| {
                        (entry_address << u8::BITS) + (*byte as usize)
                    });
                let header: *const system_description::Header =
                    entry as *const system_description::Header;
                let header: &system_description::Header = unsafe { &*header };
                (header.signature() == "APIC").then(|| {
                    let table: *const multiple_apic_description::Table =
                        entry as *const multiple_apic_description::Table;
                    unsafe { &*table }
                })
            })
            .unwrap()
    }

    pub fn madt_mut(&mut self) -> &mut multiple_apic_description::Table {
        self.bytes_mut()
            .chunks(mem::size_of::<usize>())
            .find_map(|entry_address_bytes| {
                let entry: usize = entry_address_bytes
                    .iter()
                    .rev()
                    .fold(0usize, |entry_address, byte| {
                        (entry_address << u8::BITS) + (*byte as usize)
                    });
                let header: *const system_description::Header =
                    entry as *const system_description::Header;
                let header: &system_description::Header = unsafe { &*header };
                (header.signature() == "APIC").then(|| {
                    let table: *mut multiple_apic_description::Table =
                        entry as *mut multiple_apic_description::Table;
                    unsafe { &mut *table }
                })
            })
            .unwrap()
    }

    pub fn ssdt(&self) -> &secondary_system_description::Table {
        self.bytes()
            .chunks(mem::size_of::<usize>())
            .find_map(|entry_address_bytes| {
                let entry: usize = entry_address_bytes
                    .iter()
                    .rev()
                    .fold(0usize, |entry_address, byte| {
                        (entry_address << u8::BITS) + (*byte as usize)
                    });
                let header: *const system_description::Header =
                    entry as *const system_description::Header;
                let header: &system_description::Header = unsafe { &*header };
                (header.signature() == "SSDT").then(|| {
                    let table: *const secondary_system_description::Table =
                        entry as *const secondary_system_description::Table;
                    unsafe { &*table }
                })
            })
            .unwrap()
    }

    fn bytes(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let table: usize = table as usize;
        let first_byte: usize = table + mem::size_of_val(self);
        let first_byte: *const u8 = first_byte as *const u8;
        let length: usize = self.header.table_size() - mem::size_of_val(self);
        unsafe { slice::from_raw_parts(first_byte, length) }
    }

    fn bytes_mut(&mut self) -> &mut [u8] {
        let table: *mut Self = self as *mut Self;
        let table: usize = table as usize;
        let first_byte: usize = table + mem::size_of_val(self);
        let first_byte: *mut u8 = first_byte as *mut u8;
        let length: usize = self.header.table_size() - mem::size_of_val(self);
        unsafe { slice::from_raw_parts_mut(first_byte, length) }
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
