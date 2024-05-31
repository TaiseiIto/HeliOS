mod generic_initiator;
mod generic_port;
mod gicc;
mod gic_interrupt_translation_service;
mod memory;
mod other;
mod processor_local_apic_sapic;
mod processor_local_x2apic;

use {
    alloc::vec::Vec,
    core::{
        fmt,
        mem::size_of,
        slice,
    },
    super::system_description,
};

/// # System Resource  Table (SRAT)
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.16 System Resource  Table (SRAT)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    #[allow(dead_code)]
    reserved0: [u8; 12],
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn bytes(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let table: usize = table as usize;
        let first_byte: usize = table + size_of::<Self>();
        let first_byte: *const u8 = first_byte as *const u8;
        let size: usize = self.header.table_size() - size_of::<Self>();
        unsafe {
            slice::from_raw_parts(first_byte, size)
        }
    }

    fn iter(&self) -> Structures<'_> {
        self.into()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let structures: Vec<Structure> = self
            .iter()
            .collect();
        formatter
            .debug_struct("Table")
            .field("header", &header)
            .field("structures", &structures)
            .finish()
    }
}

#[derive(Debug)]
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
    GenericInitiator(&'a generic_initiator::Structure),
    GenericPort(&'a generic_port::Structure),
    Gicc(&'a gicc::Structure),
    GicInterruptTranslationService(&'a gic_interrupt_translation_service::Structure),
    Memory(&'a memory::Structure),
    Other(&'a other::Structure),
    ProcessorLocalApicSapic(&'a processor_local_apic_sapic::Structure),
    ProcessorLocalX2apic(&'a processor_local_x2apic::Structure),
}

impl<'a> Structure<'a> {
    fn scan(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        bytes
            .first()
            .map(|structure_type| match structure_type {
                0x00 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const processor_local_apic_sapic::Structure = structure as *const processor_local_apic_sapic::Structure;
                    let structure: &processor_local_apic_sapic::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::ProcessorLocalApicSapic(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x01 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const memory::Structure = structure as *const memory::Structure;
                    let structure: &memory::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::Memory(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x02 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const processor_local_x2apic::Structure = structure as *const processor_local_x2apic::Structure;
                    let structure: &processor_local_x2apic::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::ProcessorLocalX2apic(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x03 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const gicc::Structure = structure as *const gicc::Structure;
                    let structure: &gicc::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::Gicc(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x04 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const gic_interrupt_translation_service::Structure = structure as *const gic_interrupt_translation_service::Structure;
                    let structure: &gic_interrupt_translation_service::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::GicInterruptTranslationService(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x05 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const generic_initiator::Structure = structure as *const generic_initiator::Structure;
                    let structure: &generic_initiator::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::GenericInitiator(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x06 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const generic_port::Structure = structure as *const generic_port::Structure;
                    let structure: &generic_port::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::GenericPort(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                _ => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const other::Structure = structure as *const other::Structure;
                    let structure: &other::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::Other(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                }
            })
    }

    fn size(&self) -> usize {
        match self {
            Self::GenericInitiator(structure) => structure.length(),
            Self::GenericPort(structure) => structure.length(),
            Self::Gicc(structure) => structure.length(),
            Self::GicInterruptTranslationService(structure) => structure.length(),
            Self::Memory(structure) => structure.length(),
            Self::Other(structure) => structure.length(),
            Self::ProcessorLocalApicSapic(structure) => structure.length(),
            Self::ProcessorLocalX2apic(structure) => structure.length(),
        }
    }
}

