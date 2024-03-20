use {
    core::{
        fmt,
        slice,
        str,
    },
    super::{
        fixed_acpi_description,
        root_system_description,
    },
};

/// # System Description Table Header
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.6 System Description Table Header
#[derive(Copy, Clone)]
#[repr(packed)]
pub struct Header {
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

impl Header {
    pub fn is_correct(&self) -> bool {
        let header: *const Self = self as *const Self;
        let first_byte: *const u8 = header as *const u8;
        let table: &[u8] = unsafe {
            slice::from_raw_parts(first_byte, self.length as usize)
        };
        table
            .iter()
            .fold(0x00u8, |sum, byte| sum.wrapping_add(*byte)) == 0
    }

    pub fn size(&self) -> usize {
        self.length as usize
    }

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

impl fmt::Debug for Header {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let length: u32 = self.length;
        let oem_revision: u32 = self.oem_revision;
        let creater_revision: u32 = self.creater_revision;
        formatter
            .debug_struct("SystemDescriptionHeader")
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

/// # ACPI System Description Tables
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2 ACPI System Description Tables
#[derive(Debug)]
pub enum Table<'a> {
    Fadt(&'a fixed_acpi_description::Table),
    Rsdt(&'a root_system_description::Table),
    Other(&'a Header),
}

impl Table<'_> {
    pub fn is_correct(&self) -> bool {
        match self {
            Self::Fadt(fadt) => fadt.is_correct(),
            Self::Rsdt(rsdt) => rsdt.is_correct(),
            Self::Other(header) => header.is_correct(),
        }
    }
}

impl<'a> From<&'a Header> for Table<'a> {
    fn from(header: &'a Header) -> Self {
        match header.signature() {
            "FACP" => {
                let header: *const Header = header as *const Header;
                let fadt: *const fixed_acpi_description::Table = header as *const fixed_acpi_description::Table;
                let fadt: &fixed_acpi_description::Table = unsafe {
                    &*fadt
                };
                Self::Fadt(fadt)
            },
            "RSDT" => {
                let header: *const Header = header as *const Header;
                let rsdt: *const root_system_description::Table = header as *const root_system_description::Table;
                let rsdt: &root_system_description::Table = unsafe {
                    &*rsdt
                };
                Self::Rsdt(rsdt)
            },
            _ => Self::Other(header),
        }
    }
}

