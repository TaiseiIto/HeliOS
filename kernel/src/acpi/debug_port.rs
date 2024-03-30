use {
    alloc::vec::Vec,
    core::{
        fmt,
        mem,
        slice,
    },
    super::{
        generic_address,
        system_description,
    },
};

/// # Debug Port Table 2 (DBG2)
/// ## References
/// * [Microsoft Debug Port Table 2](https://learn.microsoft.com/en-us/windows-hardware/drivers/bringup/acpi-debug-port-table)
#[repr(packed)]
pub struct Table2 {
    header: system_description::Header,
    offset_dbg_device_info: u32,
    number_dbg_device_info: u32,
}

impl Table2 {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn bytes(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let table: *const Self = unsafe {
            table.add(1)
        };
        let table: *const u8 = table as *const u8;
        let size: usize = self.header.table_size() - mem::size_of::<Self>();
        unsafe {
            slice::from_raw_parts(table, size)
        }
    }

    fn iter<'a>(&'a self) -> DeviceInformations<'a> {
        self.into()
    }
}

impl fmt::Debug for Table2 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let offset_dbg_device_info: u32 = self.offset_dbg_device_info;
        let number_dbg_device_info: u32 = self.number_dbg_device_info;
        let device_informations: Vec<&DeviceInformation> = self
            .iter()
            .collect();
        formatter
            .debug_struct("Table2")
            .field("header", &header)
            .field("offset_dbg_device_info", &offset_dbg_device_info)
            .field("number_dbg_device_info", &number_dbg_device_info)
            .field("device_informations", &device_informations)
            .finish()
    }
}

struct DeviceInformations<'a> {
    bytes: &'a [u8],
}

impl<'a> From<&'a Table2> for DeviceInformations<'a> {
    fn from(table: &'a Table2) -> Self {
        let bytes: &[u8] = table.bytes();
        Self {
            bytes,
        }
    }
}

impl<'a> Iterator for DeviceInformations<'a> {
    type Item = &'a DeviceInformation;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes: &[u8] = self.bytes;
        DeviceInformation::scan(bytes).map(|(device_information, remaining_bytes)| {
            self.bytes = remaining_bytes;
            device_information
        })
    }
}

#[repr(packed)]
struct DeviceInformation {
    revision: u8,
    length: u16,
    number_of_generic_address_registers: u8,
    namespace_string_length: u16,
    namespace_string_offset: u16,
    oem_data_length: u16,
    oem_data_offset: u16,
    port_type: u16,
    port_subtype: u16,
    reserved0: u16,
    base_address_register_offset: u16,
    address_size_offset: u16,
}

impl DeviceInformation {
    fn base_address_registers(&self) -> &[generic_address::Structure] {
        let offset: usize = self.base_address_register_offset as usize;
        let base_address_registers: &u8 = &self.bytes()[offset];
        let base_address_registers: *const u8 = base_address_registers as *const u8;
        let base_address_registers: *const generic_address::Structure = base_address_registers as *const generic_address::Structure;
        let length: usize = self.number_of_generic_address_registers as usize;
        unsafe {
            slice::from_raw_parts(base_address_registers, length)
        }
    }

    fn bytes(&self) -> &[u8] {
        let bytes: *const Self = self as *const Self;
        let bytes: *const u8 = bytes as *const u8;
        let length: usize = self.length();
        unsafe {
            slice::from_raw_parts(bytes, length)
        }
    }

    fn length(&self) -> usize {
        self.length as usize
    }

    fn scan(bytes: &[u8]) -> Option<(&Self, &[u8])> {
        bytes
            .first()
            .map(|device_information| {
                let device_information: *const u8 = device_information as *const u8;
                let device_information: *const Self = device_information as *const Self;
                let device_information: &Self = unsafe {
                    &*device_information
                };
                let remaining_bytes: &[u8] = &bytes[device_information.length()..];
                (device_information, remaining_bytes)
            })
    }
}

impl fmt::Debug for DeviceInformation {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let revision: u8 = self.revision;
        let length: u16 = self.length;
        let namespace_string_length: u16 = self.namespace_string_length;
        let namespace_string_offset: u16 = self.namespace_string_offset;
        let oem_data_length: u16 = self.oem_data_length;
        let oem_data_offset: u16 = self.oem_data_offset;
        let port_type: u16 = self.port_type;
        let port_subtype: u16 = self.port_subtype;
        let reserved0: u16 = self.reserved0;
        let address_size_offset: u16 = self.address_size_offset;
        formatter
            .debug_struct("DeviceInformation")
            .field("revision", &revision)
            .field("length", &length)
            .field("namespace_string_length", &namespace_string_length)
            .field("namespace_string_offset", &namespace_string_offset)
            .field("oem_data_length", &oem_data_length)
            .field("oem_data_offset", &oem_data_offset)
            .field("port_type", &port_type)
            .field("port_subtype", &port_subtype)
            .field("reserved0", &reserved0)
            .field("address_size_offset", &address_size_offset)
            .field("base_address_registers", &self.base_address_registers())
            .finish()
    }
}

