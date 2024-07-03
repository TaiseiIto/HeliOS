mod acpi_namespace_device_declaration;
mod hardware_static_affinity;
mod hardware_unit_definition;
mod other;
mod reserved_memory_region;
mod root_port_ats_capability;
mod soc_integrated;

use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem::size_of,
        slice,
    },
    super::system_description,
};

/// # DMA Remapping Table
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.1 DMA Remapping Reporting Structure
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    host_address_width: u8,
    flags: Flags,
    #[allow(dead_code)]
    reserved0: [u8; 10],
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn bytes(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let table: *const Self = unsafe {
            table.add(1)
        };
        let table: *const u8 = table as *const u8;
        let size: usize = self.header.table_size() - size_of::<Self>();
        unsafe {
            slice::from_raw_parts(table, size)
        }
    }

    fn iter(&self) -> Structures<'_> {
        self.into()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let host_address_width: u8 = self.host_address_width;
        let flags: Flags = self.flags;
        let structures: Vec<Structure> = self
            .iter()
            .collect();
        formatter
            .debug_struct("Table")
            .field("header", &header)
            .field("host_address_width", &host_address_width)
            .field("flags", &flags)
            .field("structures", &structures)
            .finish()
    }
}

#[bitfield(u8)]
struct Flags {
    intr_remap: bool,
    x2apic_opt_out: bool,
    dma_ctrl_platform_opt_in_flag: bool,
    #[bits(5, access = RO)]
    reserved0: u8,
}

struct Structures<'a> {
    bytes: &'a [u8],
}

impl<'a> From<&'a Table> for Structures<'a> {
    fn from(table: &'a Table) -> Self {
        let bytes: &[u8] = table.bytes();
        Self {
            bytes,
        }
    }
}

impl<'a> Iterator for Structures<'a> {
    type Item = Structure<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes: &[u8] = self.bytes;
        Self::Item::scan(bytes).map(|(structure, remaining_bytes)| {
            self.bytes = remaining_bytes;
            structure
        })
    }
}

#[derive(Debug)]
enum Structure<'a> {
    Andd(&'a acpi_namespace_device_declaration::Structure),
    Atsr(&'a root_port_ats_capability::Structure),
    Drhd(&'a hardware_unit_definition::Structure),
    Other(&'a other::Structure),
    Rhsa(&'a hardware_static_affinity::Structure),
    Rmrr(&'a reserved_memory_region::Structure),
    Satc(&'a soc_integrated::address_translation_cache::Structure),
    Sidp(&'a soc_integrated::device_property::Structure),
}

impl<'a> Structure<'a> {
    fn scan(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        bytes
            .first()
            .zip(bytes.get(1))
            .map(|(structure_type_low, structure_type_high)| {
                let structure_type: u16 = (*structure_type_low as u16) + ((*structure_type_high as u16) << u8::BITS);
                match structure_type {
                    0x0000 => {
                        let structure: *const u8 = structure_type_low as *const u8;
                        let structure: *const hardware_unit_definition::Structure = structure as *const hardware_unit_definition::Structure;
                        let structure: &hardware_unit_definition::Structure = unsafe {
                            &*structure
                        };
                        let structure = Self::Drhd(structure);
                        let remaining_bytes: &[u8] = &bytes[structure.size()..];
                        (structure, remaining_bytes)
                    },
                    0x0001 => {
                        let structure: *const u8 = structure_type_low as *const u8;
                        let structure: *const reserved_memory_region::Structure = structure as *const reserved_memory_region::Structure;
                        let structure: &reserved_memory_region::Structure = unsafe {
                            &*structure
                        };
                        let structure = Self::Rmrr(structure);
                        let remaining_bytes: &[u8] = &bytes[structure.size()..];
                        (structure, remaining_bytes)
                    },
                    0x0002 => {
                        let structure: *const u8 = structure_type_low as *const u8;
                        let structure: *const root_port_ats_capability::Structure = structure as *const root_port_ats_capability::Structure;
                        let structure: &root_port_ats_capability::Structure = unsafe {
                            &*structure
                        };
                        let structure = Self::Atsr(structure);
                        let remaining_bytes: &[u8] = &bytes[structure.size()..];
                        (structure, remaining_bytes)
                    },
                    0x0003 => {
                        let structure: *const u8 = structure_type_low as *const u8;
                        let structure: *const hardware_static_affinity::Structure = structure as *const hardware_static_affinity::Structure;
                        let structure: &hardware_static_affinity::Structure = unsafe {
                            &*structure
                        };
                        let structure = Self::Rhsa(structure);
                        let remaining_bytes: &[u8] = &bytes[structure.size()..];
                        (structure, remaining_bytes)
                    },
                    0x0004 => {
                        let structure: *const u8 = structure_type_low as *const u8;
                        let structure: *const acpi_namespace_device_declaration::Structure = structure as *const acpi_namespace_device_declaration::Structure;
                        let structure: &acpi_namespace_device_declaration::Structure = unsafe {
                            &*structure
                        };
                        let structure = Self::Andd(structure);
                        let remaining_bytes: &[u8] = &bytes[structure.size()..];
                        (structure, remaining_bytes)
                    },
                    0x0005 => {
                        let structure: *const u8 = structure_type_low as *const u8;
                        let structure: *const soc_integrated::address_translation_cache::Structure = structure as *const soc_integrated::address_translation_cache::Structure;
                        let structure: &soc_integrated::address_translation_cache::Structure = unsafe {
                            &*structure
                        };
                        let structure = Self::Satc(structure);
                        let remaining_bytes: &[u8] = &bytes[structure.size()..];
                        (structure, remaining_bytes)
                    },
                    0x0006 => {
                        let structure: *const u8 = structure_type_low as *const u8;
                        let structure: *const soc_integrated::device_property::Structure = structure as *const soc_integrated::device_property::Structure;
                        let structure: &soc_integrated::device_property::Structure = unsafe {
                            &*structure
                        };
                        let structure = Self::Sidp(structure);
                        let remaining_bytes: &[u8] = &bytes[structure.size()..];
                        (structure, remaining_bytes)
                    },
                    _ => {
                        let structure: *const u8 = structure_type_low as *const u8;
                        let structure: *const other::Structure = structure as *const other::Structure;
                        let structure: &other::Structure = unsafe {
                            &*structure
                        };
                        let structure = Self::Other(structure);
                        let remaining_bytes: &[u8] = &bytes[structure.size()..];
                        (structure, remaining_bytes)
                    },
                }
            })
    }

    fn size(&self) -> usize {
        match self {
            Self::Andd(structure) => structure.length(),
            Self::Atsr(structure) => structure.length(),
            Self::Drhd(structure) => structure.length(),
            Self::Other(structure) => structure.length(),
            Self::Rhsa(structure) => structure.length(),
            Self::Rmrr(structure) => structure.length(),
            Self::Satc(structure) => structure.length(),
            Self::Sidp(structure) => structure.length(),
        }
    }
}

