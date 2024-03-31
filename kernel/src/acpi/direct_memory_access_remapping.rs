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
        mem,
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
        let size: usize = self.header.table_size() - mem::size_of::<Self>();
        unsafe {
            slice::from_raw_parts(table, size)
        }
    }

    fn iter<'a>(&'a self) -> Structures<'a> {
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
            .get(0)
            .zip(bytes.get(1))
            .map(|(structure_type_low, structure_type_high)| {
                let structure_type: u16 = (*structure_type_low as u16) + ((*structure_type_high as u16) << u8::BITS);
                match structure_type {
                    0x0000 => {
                        let drhd: *const u8 = structure_type as *const u8;
                        let drhd: *const hardware_unit_definition::Structure = drhd as *const hardware_unit_definition::Structure;
                        let drhd: &hardware_unit_definition::Structure = unsafe {
                            &*drhd
                        };
                        let drhd = Self::Drhd(drhd);
                        let remaining_bytes: &[u8] = &bytes[drhd.size()..];
                        (drhd, remaining_bytes)
                    },
                    0x0001 => {
                        let rmrr: *const u8 = structure_type as *const u8;
                        let rmrr: *const reserved_memory_region::Structure = rmrr as *const reserved_memory_region::Structure;
                        let rmrr: &reserved_memory_region::Structure = unsafe {
                            &*rmrr
                        };
                        let rmrr = Self::Rmrr(rmrr);
                        let remaining_bytes: &[u8] = &bytes[rmrr.size()..];
                        (rmrr, remaining_bytes)
                    },
                    0x0002 => {
                        let atsr: *const u8 = structure_type as *const u8;
                        let atsr: *const root_port_ats_capability::Structure = atsr as *const root_port_ats_capability::Structure;
                        let atsr: &root_port_ats_capability::Structure = unsafe {
                            &*atsr
                        };
                        let atsr = Self::Atsr(atsr);
                        let remaining_bytes: &[u8] = &bytes[atsr.size()..];
                        (atsr, remaining_bytes)
                    },
                    0x0003 => {
                        let rhsa: *const u8 = structure_type as *const u8;
                        let rhsa: *const hardware_static_affinity::Structure = rhsa as *const hardware_static_affinity::Structure;
                        let rhsa: &hardware_static_affinity::Structure = unsafe {
                            &*rhsa
                        };
                        let rhsa = Self::Rhsa(rhsa);
                        let remaining_bytes: &[u8] = &bytes[rhsa.size()..];
                        (rhsa, remaining_bytes)
                    },
                    0x0004 => {
                        let andd: *const u8 = structure_type as *const u8;
                        let andd: *const acpi_namespace_device_declaration::Structure = andd as *const acpi_namespace_device_declaration::Structure;
                        let andd: &acpi_namespace_device_declaration::Structure = unsafe {
                            &*andd
                        };
                        let andd = Self::Andd(andd);
                        let remaining_bytes: &[u8] = &bytes[andd.size()..];
                        (andd, remaining_bytes)
                    },
                    0x0005 => {
                        let satc: *const u8 = structure_type as *const u8;
                        let satc: *const soc_integrated::address_translation_cache::Structure = satc as *const soc_integrated::address_translation_cache::Structure;
                        let satc: &soc_integrated::address_translation_cache::Structure = unsafe {
                            &*satc
                        };
                        let satc = Self::Satc(satc);
                        let remaining_bytes: &[u8] = &bytes[satc.size()..];
                        (satc, remaining_bytes)
                    },
                    0x0006 => {
                        let sidp: *const u8 = structure_type as *const u8;
                        let sidp: *const soc_integrated::device_property::Structure = sidp as *const soc_integrated::device_property::Structure;
                        let sidp: &soc_integrated::device_property::Structure = unsafe {
                            &*sidp
                        };
                        let sidp = Self::Sidp(sidp);
                        let remaining_bytes: &[u8] = &bytes[sidp.size()..];
                        (sidp, remaining_bytes)
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
                    },
                }
            })
    }

    fn size(&self) -> usize {
        match self {
            Self::Andd(andd) => andd.length(),
            Self::Atsr(atsr) => atsr.length(),
            Self::Drhd(drhd) => drhd.length(),
            Self::Other(other) => other.length(),
            Self::Rhsa(rhsa) => rhsa.length(),
            Self::Rmrr(rmrr) => rmrr.length(),
            Self::Satc(satc) => satc.length(),
            Self::Sidp(sidp) => sidp.length(),
        }
    }
}

