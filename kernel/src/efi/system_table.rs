use {
    alloc::vec::Vec,
    core::{
        fmt,
        iter,
    },
    crate::acpi,
    super::{
        BootServices,
        Guid,
        Handle,
        RuntimeServices,
        Status,
        TableHeader,
        Void,
        char16,
        configuration,
        memory,
        simple_text,
    },
};

#[macro_export]
macro_rules! efi_print {
    ($($arg:tt)*) => ($crate::efi::SystemTable::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! efi_println {
    ($fmt:expr) => (efi_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (efi_print!(concat!($fmt, "\n"), $($arg)*));
}

/// # EFI_SYSTEM_TABLE
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.3 EFI System Table
#[derive(Debug)]
#[repr(C)]
pub struct SystemTable<'a> {
    hdr: TableHeader,
    firmware_vendor: char16::NullTerminatedString<'a>,
    firmware_revision: u32,
    console_in_handle: Handle<'a>,
    con_in: &'a simple_text::input::Protocol<'a>,
    console_out_handle: Handle<'a>,
    con_out: &'a simple_text::output::Protocol<'a>,
    standard_error_handle: Handle<'a>,
    std_err: &'a simple_text::output::Protocol<'a>,
    runtime_services: &'a RuntimeServices,
    boot_services: &'a BootServices,
    configuration_tables: configuration::Tables<'a>,
}

impl SystemTable<'_> {
    #[allow(dead_code)]
    pub fn allocate_pages(&self, pages: usize) -> Result<&Void, Status> {
        self.boot_services
            .allocate_pages(pages)
            .map(|physical_address| physical_address.into())
    }

    #[allow(dead_code)]
    pub fn allocate_pool(&self, size: usize) -> Result<&Void, Status> {
        self.boot_services.allocate_pool(size)
    }

    #[allow(dead_code)]
    pub fn exit_boot_services(&self, image: Handle) -> Result<memory::Map, Status> {
        self.boot_services.exit_boot_services(image)
    }

    #[allow(dead_code)]
    pub fn free_pages(&self, virtual_address: &Void, pages: usize) -> Result<(), Status> {
        let physical_address: memory::PhysicalAddress = virtual_address.into();
        self.boot_services.free_pages(physical_address, pages)
    }

    #[allow(dead_code)]
    pub fn free_pool(&self, pool: &Void) -> Result<(), Status> {
        self.boot_services.free_pool(pool)
    }

    #[allow(dead_code)]
    pub fn locate_protocol(&self, registration: &Void, guid: Guid) -> Result<&Void, Status> {
        self.boot_services.locate_protocol(registration, guid)
    }

    #[allow(dead_code)]
    pub fn memory_map(&self) -> Result<memory::Map, Status> {
        self.boot_services.memory_map()
    }

    pub fn rsdp_mut(&mut self) -> &mut acpi::root_system_description::Pointer {
        self.configuration_tables.rsdp_mut()
    }

    pub fn shutdown(&self) {
        self.runtime_services.shutdown();
    }
}

impl fmt::Write for SystemTable<'_> {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        let string: Vec<u16> = string
            .replace('\n', "\r\n")
            .chars()
            .filter_map(|character| (character as u32).try_into().ok())
            .chain(iter::once(0))
            .collect();
        let string: char16::NullTerminatedString = (&string).into();
        self.con_out
            .output_string(string)
            .map_err(|_| fmt::Error)
    }
}

