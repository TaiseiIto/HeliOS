use {
    alloc::vec::Vec,
    core::{
        fmt,
        mem,
        str,
    },
    super::system_description,
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
    rsdt_address: u32,
    length: u32,
    xsdt_address: u64,
    extended_checksum: u8,
    reserved: [u8; 3],
}

impl Pointer {
    pub fn is_correct(&self) -> bool {
        self.checksum() && self.extended_checksum() && self.table().is_correct()
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
        let rsdp: *const [u8; mem::size_of::<Self>()] = rsdp as *const [u8; mem::size_of::<Self>()];
        let rsdp: &[u8; mem::size_of::<Self>()]  = unsafe {
            &*rsdp
        };
        rsdp.iter()
            .fold(0x00u8, |sum, byte| sum.wrapping_add(*byte)) == 0
    }

    fn oemid(&self) -> &str {
        str::from_utf8(self.oemid.as_slice()).unwrap()
    }

    fn signature(&self) -> &str {
        str::from_utf8(self.signature.as_slice()).unwrap()
    }

    fn table(&self) -> system_description::Table {
        let rsdt_header: usize = self.rsdt_address as usize;
        let rsdt_header: *const system_description::Header = rsdt_header as *const system_description::Header;
        let rsdt_header: &system_description::Header = unsafe {
            &*rsdt_header
        };
        rsdt_header.into()
    }
}

impl fmt::Debug for Pointer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let length: u32 = self.length;
        let xsdt_address: u64 = self.xsdt_address;
        formatter
            .debug_struct("RSDP")
            .field("signature", &self.signature())
            .field("checksum", &self.checksum)
            .field("oemid", &self.oemid())
            .field("revision", &self.revision)
            .field("rsdt", &self.table())
            .field("length", &length)
            .field("xsdt_address", &xsdt_address)
            .field("extended_checksum", &self.extended_checksum)
            .field("reserved", &self.reserved)
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
    pub fn entries<'a>(&'a self) -> Vec<system_description::Table<'a>> {
        let rsdt: *const Self = self as *const Self;
        let rsdt: usize = rsdt as usize;
        let first_entry: usize = rsdt + mem::size_of::<Self>();
        let first_entry: *const u32 = first_entry as *const u32;
        let entries: usize = (self.header.size() - mem::size_of::<Self>()) / 4;
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
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("entries", &self.entries())
            .finish()
    }
}

