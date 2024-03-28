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
        mem,
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
    reserved0: [u8; 12],
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }

    fn bytes(&self) -> &[u8] {
        let table: *const Self = self as *const Self;
        let table: usize = table as usize;
        let first_byte: usize = table + mem::size_of::<Self>();
        let first_byte: *const u8 = first_byte as *const u8;
        let size: usize = self.header.table_size() - mem::size_of::<Self>();
        unsafe {
            slice::from_raw_parts(first_byte, size)
        }
    }

    fn iter<'a>(&'a self) -> Structures<'a> {
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
                    let processor_local_apic_sapic: *const u8 = structure_type as *const u8;
                    let processor_local_apic_sapic: *const processor_local_apic_sapic::Structure = processor_local_apic_sapic as *const processor_local_apic_sapic::Structure;
                    let processor_local_apic_sapic: &processor_local_apic_sapic::Structure = unsafe {
                        &*processor_local_apic_sapic
                    };
                    let processor_local_apic_sapic = Self::ProcessorLocalApicSapic(processor_local_apic_sapic);
                    let remaining_bytes: &[u8] = &bytes[processor_local_apic_sapic.size()..];
                    (processor_local_apic_sapic, remaining_bytes)
                },
                0x01 => {
                    let memory: *const u8 = structure_type as *const u8;
                    let memory: *const memory::Structure = memory as *const memory::Structure;
                    let memory: &memory::Structure = unsafe {
                        &*memory
                    };
                    let memory = Self::Memory(memory);
                    let remaining_bytes: &[u8] = &bytes[memory.size()..];
                    (memory, remaining_bytes)
                },
                0x02 => {
                    let processor_local_x2apic: *const u8 = structure_type as *const u8;
                    let processor_local_x2apic: *const processor_local_x2apic::Structure = processor_local_x2apic as *const processor_local_x2apic::Structure;
                    let processor_local_x2apic: &processor_local_x2apic::Structure = unsafe {
                        &*processor_local_x2apic
                    };
                    let processor_local_x2apic = Self::ProcessorLocalX2apic(processor_local_x2apic);
                    let remaining_bytes: &[u8] = &bytes[processor_local_x2apic.size()..];
                    (processor_local_x2apic, remaining_bytes)
                },
                0x03 => {
                    let gicc: *const u8 = structure_type as *const u8;
                    let gicc: *const gicc::Structure = gicc as *const gicc::Structure;
                    let gicc: &gicc::Structure = unsafe {
                        &*gicc
                    };
                    let gicc = Self::Gicc(gicc);
                    let remaining_bytes: &[u8] = &bytes[gicc.size()..];
                    (gicc, remaining_bytes)
                },
                0x04 => {
                    let gic_interrupt_translation_service: *const u8 = structure_type as *const u8;
                    let gic_interrupt_translation_service: *const gic_interrupt_translation_service::Structure = gic_interrupt_translation_service as *const gic_interrupt_translation_service::Structure;
                    let gic_interrupt_translation_service: &gic_interrupt_translation_service::Structure = unsafe {
                        &*gic_interrupt_translation_service
                    };
                    let gic_interrupt_translation_service = Self::GicInterruptTranslationService(gic_interrupt_translation_service);
                    let remaining_bytes: &[u8] = &bytes[gic_interrupt_translation_service.size()..];
                    (gic_interrupt_translation_service, remaining_bytes)
                },
                _ => {
                    let other: *const u8 = structure_type as *const u8;
                    let other: *const other::Structure = other as *const other::Structure;
                    let other: &other::Structure = unsafe {
                        &*other
                    };
                    let other = Self::Other(other);
                    let remaining_bytes: &[u8] = &bytes[other.size()..];
                    (other, remaining_bytes)
                }
            })
    }

    fn size(&self) -> usize {
        match self {
            Self::Gicc(gicc) => gicc.length(),
            Self::GicInterruptTranslationService(gic_interrupt_translation_service) => gic_interrupt_translation_service.length(),
            Self::Memory(memory) => memory.length(),
            Self::Other(other) => other.length(),
            Self::ProcessorLocalApicSapic(processor_local_apic_sapic) => processor_local_apic_sapic.length(),
            Self::ProcessorLocalX2apic(processor_local_x2apic) => processor_local_x2apic.length(),
        }
    }
}

