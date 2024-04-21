use {
    alloc::vec::Vec,
    core::{
        fmt,
        mem,
        slice,
        str,
    },
    super::{
        generic_address,
        system_description,
    },
};

/// # Debug Port Table
/// ## References
/// * [Debug Port Specification - ARCHIVE (DBGP)](https://learn.microsoft.com/en-us/previous-versions/windows/hardware/design/dn639130(v=vs.85)?redirectedfrom=MSDN)
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    interface_type: u8,
    reserved0: [u8; 3],
    base_address: generic_address::Structure,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

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

    fn iter(&self) -> DeviceInformations<'_> {
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

/// # Debug Device Information structure
/// ## References
/// * [Microsoft Debug Port Table 2](https://learn.microsoft.com/en-us/windows-hardware/drivers/bringup/acpi-debug-port-table)
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
    fn address_sizes(&self) -> Vec<u32> {
        let offset: usize = self.address_size_offset as usize;
        let address_sizes: &u8 = &self.bytes()[offset];
        let address_sizes: *const u8 = address_sizes as *const u8;
        let length: usize = self.number_of_generic_address_registers as usize;
        let size: usize = mem::size_of::<u32>() * length;
        let address_sizes: &[u8] = unsafe {
            slice::from_raw_parts(address_sizes, size)
        };
        address_sizes
            .chunks(mem::size_of::<u32>())
            .map(|chunk| chunk
                .iter()
                .rev()
                .fold(0, |address_size, byte| (address_size << u8::BITS) + (*byte as u32)))
            .collect()
    }

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

    fn namespace_string(&self) -> &str {
        let offset: usize = self.namespace_string_offset as usize;
        let namespace_string: &u8= &self.bytes()[offset];
        let namespace_string: *const u8 = namespace_string as *const u8;
        let length: usize = self.namespace_string_length as usize;
        let namespace_string: &[u8] = unsafe {
            slice::from_raw_parts(namespace_string, length)
        };
        str::from_utf8(namespace_string).unwrap()
    }

    fn oem_data(&self) -> &[u8] {
        let offset: usize = self.oem_data_offset as usize;
        let oem_data: &u8= &self.bytes()[offset];
        let oem_data: *const u8 = oem_data as *const u8;
        let length: usize = self.oem_data_length as usize;
        unsafe {
            slice::from_raw_parts(oem_data, length)
        }
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
        let port_type: u16 = self.port_type;
        let port_subtype: u16 = self.port_subtype;
        formatter
            .debug_struct("DeviceInformation")
            .field("revision", &revision)
            .field("length", &length)
            .field("port_type", &port_type)
            .field("port_subtype", &port_subtype)
            .field("base_address_registers", &self.base_address_registers())
            .field("address_sizes", &self.address_sizes())
            .field("namespace_string", &self.namespace_string())
            .field("oem_data", &self.oem_data())
            .finish()
    }
}

