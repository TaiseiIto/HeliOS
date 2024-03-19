use core::{
    fmt,
    str,
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
        self.checksum() && self.extended_checksum()
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
        let rsdp: *const [u8; 36] = rsdp as *const [u8; 36];
        let rsdp: &[u8; 36]  = unsafe {
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

    fn table(&self) -> &Table {
        let table: usize = self.rsdt_address as usize;
        let table: *const Table = table as *const Table;
        unsafe {
            &*table
        }
    }
}

impl fmt::Debug for Pointer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rsdt_address: u32 = self.rsdt_address;
        let length: u32 = self.length;
        let xsdt_address: u64 = self.xsdt_address;
        formatter
            .debug_struct("RSDP")
            .field("signature", &self.signature())
            .field("checksum", &self.checksum)
            .field("oemid", &self.oemid())
            .field("revision", &self.revision)
            .field("rsdt", self.table())
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
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oemid: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creater_id: [u8; 4],
    creater_revision: u32,
}

impl Table {
    fn creater_id(&self) -> &str {
        str::from_utf8(self.creater_id.as_slice()).unwrap()
    }

    fn oemid(&self) -> &str {
        str::from_utf8(self.oemid.as_slice()).unwrap()
    }

    fn oem_table_id(&self) -> &str {
        str::from_utf8(self.oem_table_id.as_slice()).unwrap()
    }

    fn signature(&self) -> &str {
        str::from_utf8(self.signature.as_slice()).unwrap()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let length: u32 = self.length;
        let oem_revision: u32 = self.oem_revision;
        let creater_revision: u32 = self.creater_revision;
        formatter
            .debug_struct("RSDT")
            .field("signature", &self.signature())
            .field("length", &length)
            .field("revision", &self.revision)
            .field("checksum", &self.checksum)
            .field("oemid", &self.oemid())
            .field("oem_table_id", &self.oem_table_id())
            .field("oem_revision", &oem_revision)
            .field("creater_id", &self.creater_id())
            .field("creater_revision", &creater_revision)
            .finish()
    }
}

