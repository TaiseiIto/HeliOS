mod interrupt_source_override;
mod io_apic;
mod local_apic_nmi;
mod non_maskable_interrupt_source;
mod processor_local_apic;

use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
        slice,
    },
    super::system_description,
};

/// # MADT
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12 Multiple APIC Description Table (MADT)
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    local_interrupt_controller_address: u32,
    flags: Flags,
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

    fn iter<'a>(&'a self) -> InterruptControllerStructures<'a> {
        self.into()
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let local_interrupt_controller_address: u32 = self.local_interrupt_controller_address;
        let flags: Flags = self.flags;
        let interrupt_controller_structures: Vec<InterruptControllerStructure> = self
            .iter()
            .collect();
        formatter
            .debug_struct("Table")
            .field("header", &self.header)
            .field("local_interrupt_controller_address", &local_interrupt_controller_address)
            .field("flags", &flags)
            .field("interrupt_controller_structures", &interrupt_controller_structures)
            .finish()
    }
}

/// # Flags
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.12 Table 5.20 Multiple APIC Flags
#[bitfield(u32)]
struct Flags {
    pcat_compat: bool,
    #[bits(31, access = RO)]
    reserved0: u32,
}

struct InterruptControllerStructures<'a> {
    bytes: &'a [u8],
}

impl<'a> From<&'a Table> for InterruptControllerStructures<'a> {
    fn from(table: &'a Table) -> Self {
        let bytes: &[u8] = table.bytes();
        Self {
            bytes
        }
    }
}

impl<'a> Iterator for InterruptControllerStructures<'a> {
    type Item = InterruptControllerStructure<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes: &[u8] = self.bytes;
        Self::Item::scan(bytes).map(|(interrupt_controller_structure, remaining_bytes)| {
            self.bytes = remaining_bytes;
            interrupt_controller_structure
        })
    }
}

#[derive(Debug)]
enum InterruptControllerStructure<'a> {
    InterruptSourceOverride(&'a interrupt_source_override::Structure),
    IoApic(&'a io_apic::Structure),
    LocalApicNmi(&'a local_apic_nmi::Structure),
    NonMaskableInterruptSource(&'a non_maskable_interrupt_source::Structure),
    Other(&'a [u8]),
    ProcessorLocalApic(&'a processor_local_apic::Structure),
}

impl<'a> InterruptControllerStructure<'a> {
    fn scan(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        bytes
            .first()
            .map(|structure_type| match structure_type {
                0x00 => {
                    let processor_local_apic: *const u8 = structure_type as *const u8;
                    let processor_local_apic: *const processor_local_apic::Structure = processor_local_apic as *const processor_local_apic::Structure;
                    let processor_local_apic: &processor_local_apic::Structure = unsafe {
                        &*processor_local_apic
                    };
                    let processor_local_apic = Self::ProcessorLocalApic(processor_local_apic);
                    let remaining_bytes: &[u8] = &bytes[mem::size_of::<processor_local_apic::Structure>()..];
                    (processor_local_apic, remaining_bytes)
                },
                0x01 => {
                    let io_apic: *const u8 = structure_type as *const u8;
                    let io_apic: *const io_apic::Structure = io_apic as *const io_apic::Structure;
                    let io_apic: &io_apic::Structure = unsafe {
                        &*io_apic
                    };
                    let io_apic = Self::IoApic(io_apic);
                    let remaining_bytes: &[u8] = &bytes[mem::size_of::<io_apic::Structure>()..];
                    (io_apic, remaining_bytes)
                },
                0x02 => {
                    let interrupt_source_override: *const u8 = structure_type as *const u8;
                    let interrupt_source_override: *const interrupt_source_override::Structure = interrupt_source_override as *const interrupt_source_override::Structure;
                    let interrupt_source_override: &interrupt_source_override::Structure = unsafe {
                        &*interrupt_source_override
                    };
                    let interrupt_source_override = Self::InterruptSourceOverride(interrupt_source_override);
                    let remaining_bytes: &[u8] = &bytes[mem::size_of::<interrupt_source_override::Structure>()..];
                    (interrupt_source_override, remaining_bytes)
                },
                0x03 => {
                    let non_maskable_interrupt_source: *const u8 = structure_type as *const u8;
                    let non_maskable_interrupt_source: *const non_maskable_interrupt_source::Structure = non_maskable_interrupt_source as *const non_maskable_interrupt_source::Structure;
                    let non_maskable_interrupt_source: &non_maskable_interrupt_source::Structure = unsafe {
                        &*non_maskable_interrupt_source
                    };
                    let non_maskable_interrupt_source = Self::NonMaskableInterruptSource(non_maskable_interrupt_source);
                    let remaining_bytes: &[u8] = &bytes[mem::size_of::<non_maskable_interrupt_source::Structure>()..];
                    (non_maskable_interrupt_source, remaining_bytes)
                },
                0x04 => {
                    let local_apic_nmi: *const u8 = structure_type as *const u8;
                    let local_apic_nmi: *const local_apic_nmi::Structure = local_apic_nmi as *const local_apic_nmi::Structure;
                    let local_apic_nmi: &local_apic_nmi::Structure = unsafe {
                        &*local_apic_nmi
                    };
                    let local_apic_nmi = Self::LocalApicNmi(local_apic_nmi);
                    let remaining_bytes: &[u8] = &bytes[mem::size_of::<local_apic_nmi::Structure>()..];
                    (local_apic_nmi, remaining_bytes)
                },
                _ => {
                    let interrupt_controller_structure = Self::Other(bytes);
                    let remaining_bytes: &[u8] = &bytes[bytes.len()..];
                    (interrupt_controller_structure, remaining_bytes)
                }
            })
    }
}

