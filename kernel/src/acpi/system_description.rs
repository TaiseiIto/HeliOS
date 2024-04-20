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
        extended_system_description,
        firmware_performance_data,
        fixed_acpi_description,
        high_precision_event_timer,
        low_power_idle,
        memory_mapped_configuration,
        multiple_apic_description,
        root_system_description,
        secondary_system_description,
        static_resource_affinity,
        trusted_platform_module,
        watchdog_action,
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

    pub fn signature(&self) -> &str {
        str::from_utf8(self.signature.as_slice()).unwrap()
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
    Lpit(&'a low_power_idle::Table),
    Madt(&'a multiple_apic_description::Table),
    Mcfg(&'a memory_mapped_configuration::Table),
    Other(&'a Header),
    Rsdt(&'a root_system_description::Table),
    Srat(&'a static_resource_affinity::Table),
    Ssdt(&'a secondary_system_description::Table),
    Tpm2(&'a trusted_platform_module::Table),
    Waet(&'a windows_acpi_emulated_devices::Table),
    Wdat(&'a watchdog_action::Table),
    Wsmt(&'a windows_smm_security_mitigations::Table),
    Xsdt(&'a extended_system_description::Table),
}

impl Table<'_> {
    pub fn is_correct(&self) -> bool {
        match self {
            Self::Bgrt(table) => table.is_correct(),
            Self::Dbg2(table) => table.is_correct(),
            Self::Dbgp(table) => table.is_correct(),
            Self::Dmar(table) => table.is_correct(),
            Self::Dsdt(table) => table.is_correct(),
            Self::Fadt(table) => table.is_correct(),
            Self::Fpdt(table) => table.is_correct(),
            Self::Hpet(table) => table.is_correct(),
            Self::Lpit(table) => table.is_correct(),
            Self::Madt(table) => table.is_correct(),
            Self::Mcfg(table) => table.is_correct(),
            Self::Other(table) => table.is_correct(),
            Self::Rsdt(table) => table.is_correct(),
            Self::Srat(table) => table.is_correct(),
            Self::Ssdt(table) => table.is_correct(),
            Self::Tpm2(table) => table.is_correct(),
            Self::Waet(table) => table.is_correct(),
            Self::Wdat(table) => table.is_correct(),
            Self::Wsmt(table) => table.is_correct(),
            Self::Xsdt(table) => table.is_correct(),
        }
    }
}

impl<'a> From<&'a Header> for Table<'a> {
    fn from(header: &'a Header) -> Self {
        match header.signature() {
            "APIC" => {
                let header: *const Header = header as *const Header;
                let table: *const multiple_apic_description::Table = header as *const multiple_apic_description::Table;
                let table: &multiple_apic_description::Table = unsafe {
                    &*table
                };
                Self::Madt(table)
            },
            "BGRT" => {
                let header: *const Header = header as *const Header;
                let table: *const boot_graphics_resource::Table = header as *const boot_graphics_resource::Table;
                let table: &boot_graphics_resource::Table = unsafe {
                    &*table
                };
                Self::Bgrt(table)
            },
            "DBG2" => {
                let header: *const Header = header as *const Header;
                let table: *const debug_port::Table2 = header as *const debug_port::Table2;
                let table: &debug_port::Table2 = unsafe {
                    &*table
                };
                Self::Dbg2(table)
            },
            "DBGP" => {
                let header: *const Header = header as *const Header;
                let table: *const debug_port::Table = header as *const debug_port::Table;
                let table: &debug_port::Table = unsafe {
                    &*table
                };
                Self::Dbgp(table)
            },
            "DMAR" => {
                let header: *const Header = header as *const Header;
                let table: *const direct_memory_access_remapping::Table = header as *const direct_memory_access_remapping::Table;
                let table: &direct_memory_access_remapping::Table = unsafe {
                    &*table
                };
                Self::Dmar(table)
            },
            "DSDT" => {
                let header: *const Header = header as *const Header;
                let table: *const differentiated_system_description::Table = header as *const differentiated_system_description::Table;
                let table: &differentiated_system_description::Table = unsafe {
                    &*table
                };
                Self::Dsdt(table)
            },
            "FACP" => {
                let header: *const Header = header as *const Header;
                let table: *const fixed_acpi_description::Table = header as *const fixed_acpi_description::Table;
                let table: &fixed_acpi_description::Table = unsafe {
                    &*table
                };
                Self::Fadt(table)
            },
            // "FIDT"
            "FPDT" => {
                let header: *const Header = header as *const Header;
                let table: *const firmware_performance_data::Table = header as *const firmware_performance_data::Table;
                let table: &firmware_performance_data::Table = unsafe {
                    &*table
                };
                Self::Fpdt(table)
            },
            "HPET" => {
                let header: *const Header = header as *const Header;
                let table: *const high_precision_event_timer::Table = header as *const high_precision_event_timer::Table;
                let table: &high_precision_event_timer::Table = unsafe {
                    &*table
                };
                Self::Hpet(table)
            },
            "LPIT" => {
                let header: *const Header = header as *const Header;
                let table: *const low_power_idle::Table = header as *const low_power_idle::Table;
                let table: &low_power_idle::Table = unsafe {
                    &*table
                };
                Self::Lpit(table)
            },
            "MCFG" => {
                let header: *const Header = header as *const Header;
                let table: *const memory_mapped_configuration::Table = header as *const memory_mapped_configuration::Table;
                let table: &memory_mapped_configuration::Table = unsafe {
                    &*table
                };
                Self::Mcfg(table)
            },
            // "NPKT"
            "RSDT" => {
                let header: *const Header = header as *const Header;
                let table: *const root_system_description::Table = header as *const root_system_description::Table;
                let table: &root_system_description::Table = unsafe {
                    &*table
                };
                Self::Rsdt(table)
            },
            "SRAT" => {
                let header: *const Header = header as *const Header;
                let table: *const static_resource_affinity::Table = header as *const static_resource_affinity::Table;
                let table: &static_resource_affinity::Table = unsafe {
                    &*table
                };
                Self::Srat(table)
            },
            "SSDT" => {
                let header: *const Header = header as *const Header;
                let table: *const secondary_system_description::Table = header as *const secondary_system_description::Table;
                let table: &secondary_system_description::Table = unsafe {
                    &*table
                };
                Self::Ssdt(table)
            },
            "TPM2" => {
                let header: *const Header = header as *const Header;
                let table: *const trusted_platform_module::Table = header as *const trusted_platform_module::Table;
                let table: &trusted_platform_module::Table = unsafe {
                    &*table
                };
                Self::Tpm2(table)
            },
            "WAET" => {
                let header: *const Header = header as *const Header;
                let table: *const windows_acpi_emulated_devices::Table = header as *const windows_acpi_emulated_devices::Table;
                let table: &windows_acpi_emulated_devices::Table = unsafe {
                    &*table
                };
                Self::Waet(table)
            },
            "WDAT" => {
                let header: *const Header = header as *const Header;
                let table: *const watchdog_action::Table = header as *const watchdog_action::Table;
                let table: &watchdog_action::Table = unsafe {
                    &*table
                };
                Self::Wdat(table)
            },
            "WSMT" => {
                let header: *const Header = header as *const Header;
                let table: *const windows_smm_security_mitigations::Table = header as *const windows_smm_security_mitigations::Table;
                let table: &windows_smm_security_mitigations::Table = unsafe {
                    &*table
                };
                Self::Wsmt(table)
            },
            "XSDT" => {
                let header: *const Header = header as *const Header;
                let table: *const extended_system_description::Table = header as *const extended_system_description::Table;
                let table: &extended_system_description::Table = unsafe {
                    &*table
                };
                Self::Xsdt(table)
            },
            _ => Self::Other(header),
        }
    }
}

