use {
    core::{
        fmt,
        slice,
        str,
    },
    super::{
        boot_graphics_resource,
        debug_port,
        differentiated_system_description,
        direct_memory_access_remapping,
        firmware_performance_data,
        fixed_acpi_description,
        high_precision_event_timer,
        memory_mapped_configuration,
        multiple_apic_description,
        root_system_description,
        secondary_system_description,
        static_resource_affinity,
        windows_acpi_emulated_devices,
        windows_smm_security_mitigations,
    },
};

/// # System Description Table Header
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2.6 System Description Table Header
#[derive(Copy, Clone)]
#[repr(packed)]
pub struct Header {
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oemid: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creater_id: [u8; 4],
    creater_revision: u32,
}

impl Header {
    pub fn is_correct(&self) -> bool {
        let header: *const Self = self as *const Self;
        let first_byte: *const u8 = header as *const u8;
        let table: &[u8] = unsafe {
            slice::from_raw_parts(first_byte, self.length as usize)
        };
        table
            .iter()
            .fold(0x00u8, |sum, byte| sum.wrapping_add(*byte)) == 0
    }

    pub fn table_size(&self) -> usize {
        self.length as usize
    }

    fn creater_id(&self) -> &str {
        str::from_utf8(self.creater_id.as_slice()).unwrap()
    }

    fn oemid(&self) -> &str {
        str::from_utf8(self.oemid.as_slice()).unwrap()
    }

    fn oem_table_id(&self) -> &str {
        str::from_utf8(self.oem_table_id.as_slice()).unwrap()
    }

    fn signature(&self) -> &str {
        str::from_utf8(self.signature.as_slice()).unwrap()
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let length: u32 = self.length;
        let oem_revision: u32 = self.oem_revision;
        let creater_revision: u32 = self.creater_revision;
        formatter
            .debug_struct("Header")
            .field("signature", &self.signature())
            .field("length", &length)
            .field("revision", &self.revision)
            .field("checksum", &self.checksum)
            .field("oemid", &self.oemid())
            .field("oem_table_id", &self.oem_table_id())
            .field("oem_revision", &oem_revision)
            .field("creater_id", &self.creater_id())
            .field("creater_revision", &creater_revision)
            .finish()
    }
}

/// # ACPI System Description Tables
/// ## References
/// * [Advanced Configuration and Power Interface (ACPI) Specification](https://uefi.org/sites/default/files/resources/ACPI_Spec_6_5_Aug29.pdf) 5.2 ACPI System Description Tables
#[derive(Debug)]
pub enum Table<'a> {
    Bgrt(&'a boot_graphics_resource::Table),
    Dbg2(&'a debug_port::Table2),
    Dbgp(&'a debug_port::Table),
    Dmar(&'a direct_memory_access_remapping::Table),
    Dsdt(&'a differentiated_system_description::Table),
    Fadt(&'a fixed_acpi_description::Table),
    Fpdt(&'a firmware_performance_data::Table),
    Hpet(&'a high_precision_event_timer::Table),
    Madt(&'a multiple_apic_description::Table),
    Mcfg(&'a memory_mapped_configuration::Table),
    Other(&'a Header),
    Rsdt(&'a root_system_description::Table),
    Srat(&'a static_resource_affinity::Table),
    Ssdt(&'a secondary_system_description::Table),
    Waet(&'a windows_acpi_emulated_devices::Table),
    Wsmt(&'a windows_smm_security_mitigations::Table),
}

impl Table<'_> {
    pub fn is_correct(&self) -> bool {
        match self {
            Self::Bgrt(bgrt) => bgrt.is_correct(),
            Self::Dbg2(dbg2) => dbg2.is_correct(),
            Self::Dbgp(dbgp) => dbgp.is_correct(),
            Self::Dmar(dmar) => dmar.is_correct(),
            Self::Dsdt(dsdt) => dsdt.is_correct(),
            Self::Fadt(fadt) => fadt.is_correct(),
            Self::Fpdt(fpdt) => fpdt.is_correct(),
            Self::Hpet(hpet) => hpet.is_correct(),
            Self::Madt(madt) => madt.is_correct(),
            Self::Mcfg(mcfg) => mcfg.is_correct(),
            Self::Other(header) => header.is_correct(),
            Self::Rsdt(rsdt) => rsdt.is_correct(),
            Self::Srat(srat) => srat.is_correct(),
            Self::Ssdt(ssdt) => ssdt.is_correct(),
            Self::Waet(waet) => waet.is_correct(),
            Self::Wsmt(wsmt) => wsmt.is_correct(),
        }
    }
}

impl<'a> From<&'a Header> for Table<'a> {
    fn from(header: &'a Header) -> Self {
        match header.signature() {
            "APIC" => {
                let header: *const Header = header as *const Header;
                let madt: *const multiple_apic_description::Table = header as *const multiple_apic_description::Table;
                let madt: &multiple_apic_description::Table = unsafe {
                    &*madt
                };
                Self::Madt(madt)
            },
            "BGRT" => {
                let header: *const Header = header as *const Header;
                let bgrt: *const boot_graphics_resource::Table = header as *const boot_graphics_resource::Table;
                let bgrt: &boot_graphics_resource::Table = unsafe {
                    &*bgrt
                };
                Self::Bgrt(bgrt)
            },
            "DBG2" => {
                let header: *const Header = header as *const Header;
                let dbg2: *const debug_port::Table2 = header as *const debug_port::Table2;
                let dbg2: &debug_port::Table2 = unsafe {
                    &*dbg2
                };
                Self::Dbg2(dbg2)
            },
            "DBGP" => {
                let header: *const Header = header as *const Header;
                let dbgp: *const debug_port::Table = header as *const debug_port::Table;
                let dbgp: &debug_port::Table = unsafe {
                    &*dbgp
                };
                Self::Dbgp(dbgp)
            },
            "DMAR" => {
                let header: *const Header = header as *const Header;
                let dmar: *const direct_memory_access_remapping::Table = header as *const direct_memory_access_remapping::Table;
                let dmar: &direct_memory_access_remapping::Table = unsafe {
                    &*dmar
                };
                Self::Dmar(dmar)
            },
            "DSDT" => {
                let header: *const Header = header as *const Header;
                let dsdt: *const differentiated_system_description::Table = header as *const differentiated_system_description::Table;
                let dsdt: &differentiated_system_description::Table = unsafe {
                    &*dsdt
                };
                Self::Dsdt(dsdt)
            },
            "FACP" => {
                let header: *const Header = header as *const Header;
                let fadt: *const fixed_acpi_description::Table = header as *const fixed_acpi_description::Table;
                let fadt: &fixed_acpi_description::Table = unsafe {
                    &*fadt
                };
                Self::Fadt(fadt)
            },
            // "FIDT"
            "FPDT" => {
                let header: *const Header = header as *const Header;
                let fpdt: *const firmware_performance_data::Table = header as *const firmware_performance_data::Table;
                let fpdt: &firmware_performance_data::Table = unsafe {
                    &*fpdt
                };
                Self::Fpdt(fpdt)
            },
            "HPET" => {
                let header: *const Header = header as *const Header;
                let hpet: *const high_precision_event_timer::Table = header as *const high_precision_event_timer::Table;
                let hpet: &high_precision_event_timer::Table = unsafe {
                    &*hpet
                };
                Self::Hpet(hpet)
            },
            // "LPIT"
            "MCFG" => {
                let header: *const Header = header as *const Header;
                let mcfg: *const memory_mapped_configuration::Table = header as *const memory_mapped_configuration::Table;
                let mcfg: &memory_mapped_configuration::Table = unsafe {
                    &*mcfg
                };
                Self::Mcfg(mcfg)
            },
            // "NPKT"
            "RSDT" => {
                let header: *const Header = header as *const Header;
                let rsdt: *const root_system_description::Table = header as *const root_system_description::Table;
                let rsdt: &root_system_description::Table = unsafe {
                    &*rsdt
                };
                Self::Rsdt(rsdt)
            },
            "SRAT" => {
                let header: *const Header = header as *const Header;
                let srat: *const static_resource_affinity::Table = header as *const static_resource_affinity::Table;
                let srat: &static_resource_affinity::Table = unsafe {
                    &*srat
                };
                Self::Srat(srat)
            },
            "SSDT" => {
                let header: *const Header = header as *const Header;
                let ssdt: *const secondary_system_description::Table = header as *const secondary_system_description::Table;
                let ssdt: &secondary_system_description::Table = unsafe {
                    &*ssdt
                };
                Self::Ssdt(ssdt)
            },
            // "TPM2"
            "WAET" => {
                let header: *const Header = header as *const Header;
                let waet: *const windows_acpi_emulated_devices::Table = header as *const windows_acpi_emulated_devices::Table;
                let waet: &windows_acpi_emulated_devices::Table = unsafe {
                    &*waet
                };
                Self::Waet(waet)
            }
            // "WDAT"
            "WSMT" => {
                let header: *const Header = header as *const Header;
                let wsmt: *const windows_smm_security_mitigations::Table = header as *const windows_smm_security_mitigations::Table;
                let wsmt: &windows_smm_security_mitigations::Table = unsafe {
                    &*wsmt
                };
                Self::Wsmt(wsmt)
            }
            _ => Self::Other(header),
        }
    }
}

