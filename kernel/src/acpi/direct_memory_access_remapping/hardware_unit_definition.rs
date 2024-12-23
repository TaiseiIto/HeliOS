pub mod scope;

use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem::size_of,
        slice,
    },
    super::{
        reserved_memory_region,
        root_port_ats_capability,
        soc_integrated,
    },
};

/// # DMA Remapping Hardware Unit Definition Structure
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.3 DMA Remapping Hardware Unit Definition Structure
#[repr(packed)]
pub struct Structure {
    structure_type: u16,
    length: u16,
    flags: Flags,
    size: u8,
    segment_number: u16,
    register_base_address: u64,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }

    fn bytes(&self) -> &[u8] {
        let structure: *const Self = self as *const Self;
        let first_byte: *const Self = unsafe {
            structure.add(1)
        };
        let first_byte: *const u8 = first_byte as *const u8;
        let size: usize = self.length() - size_of::<Self>();
        unsafe {
            slice::from_raw_parts(first_byte, size)
        }
    }

    fn iter(&self) -> Scopes<'_> {
        self.into()
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let structure_type: u16 = self.structure_type;
        let length: u16 = self.length;
        let flags: Flags = self.flags;
        let size: u8 = self.size;
        let segment_number: u16 = self.segment_number;
        let register_base_address: u64 = self.register_base_address;
        let scopes: Vec<&scope::Structure> = self
            .iter()
            .collect();
        formatter
            .debug_struct("Structure")
            .field("structure_type", &structure_type)
            .field("length", &length)
            .field("flags", &flags)
            .field("size", &size)
            .field("segment_number", &segment_number)
            .field("register_base_address", &register_base_address)
            .field("scopes", &scopes)
            .finish()
    }
}

#[bitfield(u8)]
struct Flags {
    include_pci_all: bool,
    #[bits(7)]
    __: u8,
}

pub struct Scopes<'a> {
    bytes: &'a [u8],
}

impl<'a> From<&'a Structure> for Scopes<'a> {
    fn from(structure: &'a Structure) -> Self {
        let bytes: &[u8] = structure.bytes();
        Self {
            bytes,
        }
    }
}

impl<'a> From<&'a reserved_memory_region::Structure> for Scopes<'a> {
    fn from(structure: &'a reserved_memory_region::Structure) -> Self {
        let bytes: &[u8] = structure.bytes();
        Self {
            bytes,
        }
    }
}

impl<'a> From<&'a root_port_ats_capability::Structure> for Scopes<'a> {
    fn from(structure: &'a root_port_ats_capability::Structure) -> Self {
        let bytes: &[u8] = structure.bytes();
        Self {
            bytes,
        }
    }
}

impl<'a> From<&'a soc_integrated::address_translation_cache::Structure> for Scopes<'a> {
    fn from(structure: &'a soc_integrated::address_translation_cache::Structure) -> Self {
        let bytes: &[u8] = structure.bytes();
        Self {
            bytes,
        }
    }
}

impl<'a> From<&'a soc_integrated::device_property::Structure> for Scopes<'a> {
    fn from(structure: &'a soc_integrated::device_property::Structure) -> Self {
        let bytes: &[u8] = structure.bytes();
        Self {
            bytes,
        }
    }
}

impl<'a> Iterator for Scopes<'a> {
    type Item = &'a scope::Structure;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes: &[u8] = self.bytes;
        scope::Structure::scan(bytes).map(|(scope, remaining_bytes)| {
            self.bytes = remaining_bytes;
            scope
        })
    }
}

