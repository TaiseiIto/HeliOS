mod bridge_io_programmable_interrupt_controller;
mod core_programmable_interrupt_controller;
mod extended_io_programmable_interrupt_controller;
mod gic_cpu_interface;
mod gic_distributer;
mod gic_interrupt_translation_service;
mod gic_msi_frame;
mod gic_redistributor;
mod hyper_transport_programmable_interrupt_controller;
mod interrupt_source_override;
mod io_apic;
mod io_sapic;
mod legacy_io_programmable_interrupt_controller;
mod local_apic_address_override;
mod local_apic_nmi;
mod local_sapic;
mod local_x2apic_nmi;
mod low_pin_count_programmable_interrupt_controller;
mod msi_programmable_interrupt_controller;
mod multiprocessor_wakeup;
mod non_maskable_interrupt_source;
mod other;
mod platform_interrupt_sources;
mod processor_local_apic;
mod processor_local_x2apic;

use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::{
        fmt,
        mem,
        slice,
    },
    crate::interrupt,
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
        let table: *const Self = unsafe {
            table.add(1)
        };
        let table: *const u8 = table as *const u8;
        let size: usize = self.header.table_size() - mem::size_of::<Self>();
        unsafe {
            slice::from_raw_parts(table, size)
        }
    }

    fn iter<'a>(&'a self) -> InterruptControllerStructures<'a> {
        self.into()
    }

    fn local_interrupt_controller(&self) -> &interrupt::apic::local::Registers {
        let local_interrupt_controller: u32 = self.local_interrupt_controller_address;
        let local_interrupt_controller: usize = local_interrupt_controller as usize;
        let local_interrupt_controller: *const interrupt::apic::local::Registers = local_interrupt_controller as *const interrupt::apic::local::Registers;
        unsafe {
            &*local_interrupt_controller
        }
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: system_description::Header = self.header;
        let flags: Flags = self.flags;
        let interrupt_controller_structures: Vec<InterruptControllerStructure> = self
            .iter()
            .collect();
        formatter
            .debug_struct("Table")
            .field("header", &header)
            .field("local_interrupt_controller_address", self.local_interrupt_controller())
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
            bytes,
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
    BridgeIoProgrammableInterruptController(&'a bridge_io_programmable_interrupt_controller::Structure),
    CoreProgrammableInterruptController(&'a core_programmable_interrupt_controller::Structure),
    ExtendedIoProgrammableInterruptController(&'a extended_io_programmable_interrupt_controller::Structure),
    GicCpuInterface(&'a gic_cpu_interface::Structure),
    GicDistributer(&'a gic_distributer::Structure),
    GicInterruptTranslationService(&'a gic_interrupt_translation_service::Structure),
    GicMsiFrame(&'a gic_msi_frame::Structure),
    GicRedistributor(&'a gic_redistributor::Structure),
    HyperTransportProgrammableInterruptController(&'a hyper_transport_programmable_interrupt_controller::Structure),
    InterruptSourceOverride(&'a interrupt_source_override::Structure),
    IoApic(&'a io_apic::Structure),
    IoSapic(&'a io_sapic::Structure),
    LegacyIoProgrammableInterruptController(&'a legacy_io_programmable_interrupt_controller::Structure),
    LocalApicAddressOverride(&'a local_apic_address_override::Structure),
    LocalApicNmi(&'a local_apic_nmi::Structure),
    LocalSapic(&'a local_sapic::Structure),
    LocalX2apicNmi(&'a local_x2apic_nmi::Structure),
    LowPinCountProgrammableInterruptController(&'a low_pin_count_programmable_interrupt_controller::Structure),
    MsiProgrammableInterruptController(&'a msi_programmable_interrupt_controller::Structure),
    MultiprocessorWakeup(&'a multiprocessor_wakeup::Structure),
    NonMaskableInterruptSource(&'a non_maskable_interrupt_source::Structure),
    Other(&'a other::Structure),
    PlatformInterruptSources(&'a platform_interrupt_sources::Structure),
    ProcessorLocalApic(&'a processor_local_apic::Structure),
    ProcessorLocalX2apic(&'a processor_local_x2apic::Structure),
}

impl<'a> InterruptControllerStructure<'a> {
    fn scan(bytes: &'a [u8]) -> Option<(Self, &'a [u8])> {
        bytes
            .first()
            .map(|structure_type| match structure_type {
                0x00 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const processor_local_apic::Structure = structure as *const processor_local_apic::Structure;
                    let structure: &processor_local_apic::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::ProcessorLocalApic(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x01 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const io_apic::Structure = structure as *const io_apic::Structure;
                    let structure: &io_apic::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::IoApic(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x02 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const interrupt_source_override::Structure = structure as *const interrupt_source_override::Structure;
                    let structure: &interrupt_source_override::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::InterruptSourceOverride(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x03 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const non_maskable_interrupt_source::Structure = structure as *const non_maskable_interrupt_source::Structure;
                    let structure: &non_maskable_interrupt_source::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::NonMaskableInterruptSource(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x04 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const local_apic_nmi::Structure = structure as *const local_apic_nmi::Structure;
                    let structure: &local_apic_nmi::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::LocalApicNmi(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x05 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const local_apic_address_override::Structure = structure as *const local_apic_address_override::Structure;
                    let structure: &local_apic_address_override::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::LocalApicAddressOverride(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x06 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const io_sapic::Structure = structure as *const io_sapic::Structure;
                    let structure: &io_sapic::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::IoSapic(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x07 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const local_sapic::Structure = structure as *const local_sapic::Structure;
                    let structure: &local_sapic::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::LocalSapic(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x08 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const platform_interrupt_sources::Structure = structure as *const platform_interrupt_sources::Structure;
                    let structure: &platform_interrupt_sources::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::PlatformInterruptSources(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x09 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const processor_local_x2apic::Structure = structure as *const processor_local_x2apic::Structure;
                    let structure: &processor_local_x2apic::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::ProcessorLocalX2apic(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x0a => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const local_x2apic_nmi::Structure = structure as *const local_x2apic_nmi::Structure;
                    let structure: &local_x2apic_nmi::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::LocalX2apicNmi(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x0b => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const gic_cpu_interface::Structure = structure as *const gic_cpu_interface::Structure;
                    let structure: &gic_cpu_interface::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::GicCpuInterface(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x0c => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const gic_distributer::Structure = structure as *const gic_distributer::Structure;
                    let structure: &gic_distributer::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::GicDistributer(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x0d => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const gic_msi_frame::Structure = structure as *const gic_msi_frame::Structure;
                    let structure: &gic_msi_frame::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::GicMsiFrame(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x0e => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const gic_redistributor::Structure = structure as *const gic_redistributor::Structure;
                    let structure: &gic_redistributor::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::GicRedistributor(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x0f => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const gic_interrupt_translation_service::Structure = structure as *const gic_interrupt_translation_service::Structure;
                    let structure: &gic_interrupt_translation_service::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::GicInterruptTranslationService(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x10 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const multiprocessor_wakeup::Structure = structure as *const multiprocessor_wakeup::Structure;
                    let structure: &multiprocessor_wakeup::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::MultiprocessorWakeup(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x11 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const core_programmable_interrupt_controller::Structure = structure as *const core_programmable_interrupt_controller::Structure;
                    let structure: &core_programmable_interrupt_controller::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::CoreProgrammableInterruptController(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x12 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const legacy_io_programmable_interrupt_controller::Structure = structure as *const legacy_io_programmable_interrupt_controller::Structure;
                    let structure: &legacy_io_programmable_interrupt_controller::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::LegacyIoProgrammableInterruptController(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x13 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const hyper_transport_programmable_interrupt_controller::Structure = structure as *const hyper_transport_programmable_interrupt_controller::Structure;
                    let structure: &hyper_transport_programmable_interrupt_controller::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::HyperTransportProgrammableInterruptController(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x14 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const extended_io_programmable_interrupt_controller::Structure = structure as *const extended_io_programmable_interrupt_controller::Structure;
                    let structure: &extended_io_programmable_interrupt_controller::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::ExtendedIoProgrammableInterruptController(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x15 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const msi_programmable_interrupt_controller::Structure = structure as *const msi_programmable_interrupt_controller::Structure;
                    let structure: &msi_programmable_interrupt_controller::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::MsiProgrammableInterruptController(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x16 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const bridge_io_programmable_interrupt_controller::Structure = structure as *const bridge_io_programmable_interrupt_controller::Structure;
                    let structure: &bridge_io_programmable_interrupt_controller::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::BridgeIoProgrammableInterruptController(structure);
                    let remaining_bytes: &[u8] = &bytes[structure.size()..];
                    (structure, remaining_bytes)
                },
                0x17 => {
                    let structure: *const u8 = structure_type as *const u8;
                    let structure: *const low_pin_count_programmable_interrupt_controller::Structure = structure as *const low_pin_count_programmable_interrupt_controller::Structure;
                    let structure: &low_pin_count_programmable_interrupt_controller::Structure = unsafe {
                        &*structure
                    };
                    let structure = Self::LowPinCountProgrammableInterruptController(structure);
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
            Self::BridgeIoProgrammableInterruptController(structure) => structure.length(),
            Self::CoreProgrammableInterruptController(structure) => structure.length(),
            Self::ExtendedIoProgrammableInterruptController(structure) => structure.length(),
            Self::GicCpuInterface(structure) => structure.length(),
            Self::GicDistributer(structure) => structure.length(),
            Self::GicInterruptTranslationService(structure) => structure.length(),
            Self::GicMsiFrame(structure) => structure.length(),
            Self::GicRedistributor(structure) => structure.length(),
            Self::HyperTransportProgrammableInterruptController(structure) => structure.length(),
            Self::InterruptSourceOverride(structure) => structure.length(),
            Self::IoApic(structure) => structure.length(),
            Self::IoSapic(structure) => structure.length(),
            Self::LegacyIoProgrammableInterruptController(structure) => structure.length(),
            Self::LocalApicAddressOverride(structure) => structure.length(),
            Self::LocalApicNmi(structure) => structure.length(),
            Self::LocalSapic(structure) => structure.length(),
            Self::LocalX2apicNmi(structure) => structure.length(),
            Self::LowPinCountProgrammableInterruptController(structure) => structure.length(),
            Self::MsiProgrammableInterruptController(structure) => structure.length(),
            Self::MultiprocessorWakeup(structure) => structure.length(),
            Self::NonMaskableInterruptSource(structure) => structure.length(),
            Self::Other(structure) => structure.length(),
            Self::PlatformInterruptSources(structure) => structure.length(),
            Self::ProcessorLocalApic(structure) => structure.length(),
            Self::ProcessorLocalX2apic(structure) => structure.length(),
        }
    }
}

