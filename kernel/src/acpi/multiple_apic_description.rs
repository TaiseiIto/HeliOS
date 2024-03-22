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
                    let processor_local_apic: *const u8 = structure_type as *const u8;
                    let processor_local_apic: *const processor_local_apic::Structure = processor_local_apic as *const processor_local_apic::Structure;
                    let processor_local_apic: &processor_local_apic::Structure = unsafe {
                        &*processor_local_apic
                    };
                    let processor_local_apic = Self::ProcessorLocalApic(processor_local_apic);
                    let remaining_bytes: &[u8] = &bytes[processor_local_apic.size()..];
                    (processor_local_apic, remaining_bytes)
                },
                0x01 => {
                    let io_apic: *const u8 = structure_type as *const u8;
                    let io_apic: *const io_apic::Structure = io_apic as *const io_apic::Structure;
                    let io_apic: &io_apic::Structure = unsafe {
                        &*io_apic
                    };
                    let io_apic = Self::IoApic(io_apic);
                    let remaining_bytes: &[u8] = &bytes[io_apic.size()..];
                    (io_apic, remaining_bytes)
                },
                0x02 => {
                    let interrupt_source_override: *const u8 = structure_type as *const u8;
                    let interrupt_source_override: *const interrupt_source_override::Structure = interrupt_source_override as *const interrupt_source_override::Structure;
                    let interrupt_source_override: &interrupt_source_override::Structure = unsafe {
                        &*interrupt_source_override
                    };
                    let interrupt_source_override = Self::InterruptSourceOverride(interrupt_source_override);
                    let remaining_bytes: &[u8] = &bytes[interrupt_source_override.size()..];
                    (interrupt_source_override, remaining_bytes)
                },
                0x03 => {
                    let non_maskable_interrupt_source: *const u8 = structure_type as *const u8;
                    let non_maskable_interrupt_source: *const non_maskable_interrupt_source::Structure = non_maskable_interrupt_source as *const non_maskable_interrupt_source::Structure;
                    let non_maskable_interrupt_source: &non_maskable_interrupt_source::Structure = unsafe {
                        &*non_maskable_interrupt_source
                    };
                    let non_maskable_interrupt_source = Self::NonMaskableInterruptSource(non_maskable_interrupt_source);
                    let remaining_bytes: &[u8] = &bytes[non_maskable_interrupt_source.size()..];
                    (non_maskable_interrupt_source, remaining_bytes)
                },
                0x04 => {
                    let local_apic_nmi: *const u8 = structure_type as *const u8;
                    let local_apic_nmi: *const local_apic_nmi::Structure = local_apic_nmi as *const local_apic_nmi::Structure;
                    let local_apic_nmi: &local_apic_nmi::Structure = unsafe {
                        &*local_apic_nmi
                    };
                    let local_apic_nmi = Self::LocalApicNmi(local_apic_nmi);
                    let remaining_bytes: &[u8] = &bytes[local_apic_nmi.size()..];
                    (local_apic_nmi, remaining_bytes)
                },
                0x05 => {
                    let local_apic_address_override: *const u8 = structure_type as *const u8;
                    let local_apic_address_override: *const local_apic_address_override::Structure = local_apic_address_override as *const local_apic_address_override::Structure;
                    let local_apic_address_override: &local_apic_address_override::Structure = unsafe {
                        &*local_apic_address_override
                    };
                    let local_apic_address_override = Self::LocalApicAddressOverride(local_apic_address_override);
                    let remaining_bytes: &[u8] = &bytes[local_apic_address_override.size()..];
                    (local_apic_address_override, remaining_bytes)
                },
                0x06 => {
                    let io_sapic: *const u8 = structure_type as *const u8;
                    let io_sapic: *const io_sapic::Structure = io_sapic as *const io_sapic::Structure;
                    let io_sapic: &io_sapic::Structure = unsafe {
                        &*io_sapic
                    };
                    let io_sapic = Self::IoSapic(io_sapic);
                    let remaining_bytes: &[u8] = &bytes[io_sapic.size()..];
                    (io_sapic, remaining_bytes)
                },
                0x07 => {
                    let local_sapic: *const u8 = structure_type as *const u8;
                    let local_sapic: *const local_sapic::Structure = local_sapic as *const local_sapic::Structure;
                    let local_sapic: &local_sapic::Structure = unsafe {
                        &*local_sapic
                    };
                    let local_sapic = Self::LocalSapic(local_sapic);
                    let remaining_bytes: &[u8] = &bytes[local_sapic.size()..];
                    (local_sapic, remaining_bytes)
                },
                0x08 => {
                    let platform_interrupt_sources: *const u8 = structure_type as *const u8;
                    let platform_interrupt_sources: *const platform_interrupt_sources::Structure = platform_interrupt_sources as *const platform_interrupt_sources::Structure;
                    let platform_interrupt_sources: &platform_interrupt_sources::Structure = unsafe {
                        &*platform_interrupt_sources
                    };
                    let platform_interrupt_sources = Self::PlatformInterruptSources(platform_interrupt_sources);
                    let remaining_bytes: &[u8] = &bytes[platform_interrupt_sources.size()..];
                    (platform_interrupt_sources, remaining_bytes)
                },
                0x09 => {
                    let processor_local_x2apic: *const u8 = structure_type as *const u8;
                    let processor_local_x2apic: *const processor_local_x2apic::Structure = processor_local_x2apic as *const processor_local_x2apic::Structure;
                    let processor_local_x2apic: &processor_local_x2apic::Structure = unsafe {
                        &*processor_local_x2apic
                    };
                    let processor_local_x2apic = Self::ProcessorLocalX2apic(processor_local_x2apic);
                    let remaining_bytes: &[u8] = &bytes[processor_local_x2apic.size()..];
                    (processor_local_x2apic, remaining_bytes)
                },
                0x0a => {
                    let local_x2apic_nmi: *const u8 = structure_type as *const u8;
                    let local_x2apic_nmi: *const local_x2apic_nmi::Structure = local_x2apic_nmi as *const local_x2apic_nmi::Structure;
                    let local_x2apic_nmi: &local_x2apic_nmi::Structure = unsafe {
                        &*local_x2apic_nmi
                    };
                    let local_x2apic_nmi = Self::LocalX2apicNmi(local_x2apic_nmi);
                    let remaining_bytes: &[u8] = &bytes[local_x2apic_nmi.size()..];
                    (local_x2apic_nmi, remaining_bytes)
                },
                0x0b => {
                    let gic_cpu_interface: *const u8 = structure_type as *const u8;
                    let gic_cpu_interface: *const gic_cpu_interface::Structure = gic_cpu_interface as *const gic_cpu_interface::Structure;
                    let gic_cpu_interface: &gic_cpu_interface::Structure = unsafe {
                        &*gic_cpu_interface
                    };
                    let gic_cpu_interface = Self::GicCpuInterface(gic_cpu_interface);
                    let remaining_bytes: &[u8] = &bytes[gic_cpu_interface.size()..];
                    (gic_cpu_interface, remaining_bytes)
                },
                0x0c => {
                    let gic_distributer: *const u8 = structure_type as *const u8;
                    let gic_distributer: *const gic_distributer::Structure = gic_distributer as *const gic_distributer::Structure;
                    let gic_distributer: &gic_distributer::Structure = unsafe {
                        &*gic_distributer
                    };
                    let gic_distributer = Self::GicDistributer(gic_distributer);
                    let remaining_bytes: &[u8] = &bytes[gic_distributer.size()..];
                    (gic_distributer, remaining_bytes)
                },
                0x0d => {
                    let gic_msi_frame: *const u8 = structure_type as *const u8;
                    let gic_msi_frame: *const gic_msi_frame::Structure = gic_msi_frame as *const gic_msi_frame::Structure;
                    let gic_msi_frame: &gic_msi_frame::Structure = unsafe {
                        &*gic_msi_frame
                    };
                    let gic_msi_frame = Self::GicMsiFrame(gic_msi_frame);
                    let remaining_bytes: &[u8] = &bytes[gic_msi_frame.size()..];
                    (gic_msi_frame, remaining_bytes)
                },
                0x0e => {
                    let gic_redistributor: *const u8 = structure_type as *const u8;
                    let gic_redistributor: *const gic_redistributor::Structure = gic_redistributor as *const gic_redistributor::Structure;
                    let gic_redistributor: &gic_redistributor::Structure = unsafe {
                        &*gic_redistributor
                    };
                    let gic_redistributor = Self::GicRedistributor(gic_redistributor);
                    let remaining_bytes: &[u8] = &bytes[gic_redistributor.size()..];
                    (gic_redistributor, remaining_bytes)
                },
                0x0f => {
                    let gic_interrupt_translation_service: *const u8 = structure_type as *const u8;
                    let gic_interrupt_translation_service: *const gic_interrupt_translation_service::Structure = gic_interrupt_translation_service as *const gic_interrupt_translation_service::Structure;
                    let gic_interrupt_translation_service: &gic_interrupt_translation_service::Structure = unsafe {
                        &*gic_interrupt_translation_service
                    };
                    let gic_interrupt_translation_service = Self::GicInterruptTranslationService(gic_interrupt_translation_service);
                    let remaining_bytes: &[u8] = &bytes[gic_interrupt_translation_service.size()..];
                    (gic_interrupt_translation_service, remaining_bytes)
                },
                0x10 => {
                    let multiprocessor_wakeup: *const u8 = structure_type as *const u8;
                    let multiprocessor_wakeup: *const multiprocessor_wakeup::Structure = multiprocessor_wakeup as *const multiprocessor_wakeup::Structure;
                    let multiprocessor_wakeup: &multiprocessor_wakeup::Structure = unsafe {
                        &*multiprocessor_wakeup
                    };
                    let multiprocessor_wakeup = Self::MultiprocessorWakeup(multiprocessor_wakeup);
                    let remaining_bytes: &[u8] = &bytes[multiprocessor_wakeup.size()..];
                    (multiprocessor_wakeup, remaining_bytes)
                },
                0x11 => {
                    let core_programmable_interrupt_controller: *const u8 = structure_type as *const u8;
                    let core_programmable_interrupt_controller: *const core_programmable_interrupt_controller::Structure = core_programmable_interrupt_controller as *const core_programmable_interrupt_controller::Structure;
                    let core_programmable_interrupt_controller: &core_programmable_interrupt_controller::Structure = unsafe {
                        &*core_programmable_interrupt_controller
                    };
                    let core_programmable_interrupt_controller = Self::CoreProgrammableInterruptController(core_programmable_interrupt_controller);
                    let remaining_bytes: &[u8] = &bytes[core_programmable_interrupt_controller.size()..];
                    (core_programmable_interrupt_controller, remaining_bytes)
                },
                0x12 => {
                    let legacy_io_programmable_interrupt_controller: *const u8 = structure_type as *const u8;
                    let legacy_io_programmable_interrupt_controller: *const legacy_io_programmable_interrupt_controller::Structure = legacy_io_programmable_interrupt_controller as *const legacy_io_programmable_interrupt_controller::Structure;
                    let legacy_io_programmable_interrupt_controller: &legacy_io_programmable_interrupt_controller::Structure = unsafe {
                        &*legacy_io_programmable_interrupt_controller
                    };
                    let legacy_io_programmable_interrupt_controller = Self::LegacyIoProgrammableInterruptController(legacy_io_programmable_interrupt_controller);
                    let remaining_bytes: &[u8] = &bytes[legacy_io_programmable_interrupt_controller.size()..];
                    (legacy_io_programmable_interrupt_controller, remaining_bytes)
                },
                0x13 => {
                    let hyper_transport_programmable_interrupt_controller: *const u8 = structure_type as *const u8;
                    let hyper_transport_programmable_interrupt_controller: *const hyper_transport_programmable_interrupt_controller::Structure = hyper_transport_programmable_interrupt_controller as *const hyper_transport_programmable_interrupt_controller::Structure;
                    let hyper_transport_programmable_interrupt_controller: &hyper_transport_programmable_interrupt_controller::Structure = unsafe {
                        &*hyper_transport_programmable_interrupt_controller
                    };
                    let hyper_transport_programmable_interrupt_controller = Self::HyperTransportProgrammableInterruptController(hyper_transport_programmable_interrupt_controller);
                    let remaining_bytes: &[u8] = &bytes[hyper_transport_programmable_interrupt_controller.size()..];
                    (hyper_transport_programmable_interrupt_controller, remaining_bytes)
                },
                0x14 => {
                    let extended_io_programmable_interrupt_controller: *const u8 = structure_type as *const u8;
                    let extended_io_programmable_interrupt_controller: *const extended_io_programmable_interrupt_controller::Structure = extended_io_programmable_interrupt_controller as *const extended_io_programmable_interrupt_controller::Structure;
                    let extended_io_programmable_interrupt_controller: &extended_io_programmable_interrupt_controller::Structure = unsafe {
                        &*extended_io_programmable_interrupt_controller
                    };
                    let extended_io_programmable_interrupt_controller = Self::ExtendedIoProgrammableInterruptController(extended_io_programmable_interrupt_controller);
                    let remaining_bytes: &[u8] = &bytes[extended_io_programmable_interrupt_controller.size()..];
                    (extended_io_programmable_interrupt_controller, remaining_bytes)
                },
                0x15 => {
                    let msi_programmable_interrupt_controller: *const u8 = structure_type as *const u8;
                    let msi_programmable_interrupt_controller: *const msi_programmable_interrupt_controller::Structure = msi_programmable_interrupt_controller as *const msi_programmable_interrupt_controller::Structure;
                    let msi_programmable_interrupt_controller: &msi_programmable_interrupt_controller::Structure = unsafe {
                        &*msi_programmable_interrupt_controller
                    };
                    let msi_programmable_interrupt_controller = Self::MsiProgrammableInterruptController(msi_programmable_interrupt_controller);
                    let remaining_bytes: &[u8] = &bytes[msi_programmable_interrupt_controller.size()..];
                    (msi_programmable_interrupt_controller, remaining_bytes)
                },
                0x16 => {
                    let bridge_io_programmable_interrupt_controller: *const u8 = structure_type as *const u8;
                    let bridge_io_programmable_interrupt_controller: *const bridge_io_programmable_interrupt_controller::Structure = bridge_io_programmable_interrupt_controller as *const bridge_io_programmable_interrupt_controller::Structure;
                    let bridge_io_programmable_interrupt_controller: &bridge_io_programmable_interrupt_controller::Structure = unsafe {
                        &*bridge_io_programmable_interrupt_controller
                    };
                    let bridge_io_programmable_interrupt_controller = Self::BridgeIoProgrammableInterruptController(bridge_io_programmable_interrupt_controller);
                    let remaining_bytes: &[u8] = &bytes[bridge_io_programmable_interrupt_controller.size()..];
                    (bridge_io_programmable_interrupt_controller, remaining_bytes)
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
            Self::BridgeIoProgrammableInterruptController(bridge_io_programmable_interrupt_controller) => bridge_io_programmable_interrupt_controller.length(),
            Self::CoreProgrammableInterruptController(core_programmable_interrupt_controller) => core_programmable_interrupt_controller.length(),
            Self::ExtendedIoProgrammableInterruptController(extended_io_programmable_interrupt_controller) => extended_io_programmable_interrupt_controller.length(),
            Self::GicCpuInterface(gic_cpu_interface) => gic_cpu_interface.length(),
            Self::GicDistributer(gic_distributer) => gic_distributer.length(),
            Self::GicInterruptTranslationService(gic_interrupt_translation_service) => gic_interrupt_translation_service.length(),
            Self::GicMsiFrame(gic_msi_frame) => gic_msi_frame.length(),
            Self::GicRedistributor(gic_redistributor) => gic_redistributor.length(),
            Self::HyperTransportProgrammableInterruptController(hyper_transport_programmable_interrupt_controller) => hyper_transport_programmable_interrupt_controller.length(),
            Self::InterruptSourceOverride(interrupt_source_override) => interrupt_source_override.length(),
            Self::IoApic(io_apic) => io_apic.length(),
            Self::IoSapic(io_sapic) => io_sapic.length(),
            Self::LegacyIoProgrammableInterruptController(legacy_io_programmable_interrupt_controller) => legacy_io_programmable_interrupt_controller.length(),
            Self::LocalApicAddressOverride(local_apic_address_override) => local_apic_address_override.length(),
            Self::LocalApicNmi(local_apic_nmi) => local_apic_nmi.length(),
            Self::LocalSapic(local_sapic) => local_sapic.length(),
            Self::LocalX2apicNmi(local_x2apic_nmi) => local_x2apic_nmi.length(),
            Self::MsiProgrammableInterruptController(msi_programmable_interrupt_controller) => msi_programmable_interrupt_controller.length(),
            Self::MultiprocessorWakeup(multiprocessor_wakeup) => multiprocessor_wakeup.length(),
            Self::NonMaskableInterruptSource(non_maskable_interrupt_source) => non_maskable_interrupt_source.length(),
            Self::Other(other) => other.length(),
            Self::PlatformInterruptSources(platform_interrupt_sources) => platform_interrupt_sources.length(),
            Self::ProcessorLocalApic(processor_local_apic) => processor_local_apic.length(),
            Self::ProcessorLocalX2apic(processor_local_x2apic) => processor_local_x2apic.length(),
        }
    }
}

