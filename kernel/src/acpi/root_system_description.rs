use {
    alloc::vec::Vec,
    core::{
        fmt,
        mem::size_of,
        str,
    },
    super::{
        extended_system_description,
        system_description,
    },
};

/// # RSDP
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.5.3 Root System Description Pointer (RSDP) Structure
#[repr(packed)]
pub struct Pointer {
    signature: [u8; 8],
    checksum: u8,
    oemid: [u8; 6],
    revision: u8,
    rsdt: u32,
    length: u32,
    xsdt: u64,
    extended_checksum: u8,
    #[allow(dead_code)]
    reserved0: [u8; 3],
}

impl Pointer {
    pub fn is_correct(&self) -> bool {
        self.checksum() && self.extended_checksum() && self.rsdt().is_correct() && self.xsdt().is_correct()
    }

    pub fn oemid(&self) -> &str {
        str::from_utf8(self.oemid.as_slice()).unwrap()
    }

    pub fn xsdt(&self) -> &extended_system_description::Table {
        let xsdt: usize = self.xsdt as usize;
        let xsdt: *const extended_system_description::Table = xsdt as *const extended_system_description::Table;
        unsafe {
            &*xsdt
        }
    }

    pub fn xsdt_mut(&mut self) -> &mut extended_system_description::Table {
        let xsdt: usize = self.xsdt as usize;
        let xsdt: *mut extended_system_description::Table = xsdt as *mut extended_system_description::Table;
        unsafe {
            &mut *xsdt
        }
    }

    fn checksum(&self) -> bool {
        let rsdp: *const Self = self as *const Self;
        let rsdp: *const [u8; 20] = rsdp as *const [u8; 20];
        let rsdp: &[u8; 20] = unsafe {
            &*rsdp
        };
        rsdp.iter()
            .fold(0x00u8, |sum, byte| sum.wrapping_add(*byte)) == 0
    }

    fn extended_checksum(&self) -> bool {
        let rsdp: *const Self = self as *const Self;
        let rsdp: *const [u8; size_of::<Self>()] = rsdp as *const [u8; size_of::<Self>()];
        let rsdp: &[u8; size_of::<Self>()]  = unsafe {
            &*rsdp
        };
        rsdp.iter()
            .fold(0x00u8, |sum, byte| sum.wrapping_add(*byte)) == 0
    }

    fn signature(&self) -> &str {
        str::from_utf8(self.signature.as_slice()).unwrap()
    }

    fn rsdt(&self) -> system_description::Table {
        let rsdt_header: usize = self.rsdt as usize;
        let rsdt_header: *const system_description::Header = rsdt_header as *const system_description::Header;
        let rsdt_header: &system_description::Header = unsafe {
            &*rsdt_header
        };
        rsdt_header.into()
    }
}

impl fmt::Debug for Pointer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let signature: &str = self.signature();
        let checksum: u8 = self.checksum;
        let oemid: &str = self.oemid();
        let revision: u8 = self.revision;
        let rsdt: system_description::Table = self.rsdt();
        let length: u32 = self.length;
        let xsdt: &extended_system_description::Table = self.xsdt();
        let extended_checksum: u8 = self.extended_checksum;
        formatter
            .debug_struct("Rsdp")
            .field("signature", &signature)
            .field("checksum", &checksum)
            .field("oemid", &oemid)
            .field("revision", &revision)
            .field("rsdt", &rsdt)
            .field("length", &length)
            .field("xsdt", &xsdt)
            .field("extended_checksum", &extended_checksum)
            .finish()
    }
}

/// # RSDT
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.7 Root System Description Table (RSDT)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
}

impl Table {
    pub fn entries(&self) -> Vec<system_description::Table<'_>> {
        let table: *const Self = self as *const Self;
        let table: usize = table as usize;
        let first_entry: usize = table + size_of::<Self>();
        let first_entry: *const u32 = first_entry as *const u32;
        let entries: usize = (self.header.table_size() - size_of::<Self>()) / size_of::<u32>();
        (0..entries)
            .map(|index| {
                let entry: u32 = unsafe {
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

