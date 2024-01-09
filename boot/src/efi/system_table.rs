use {
    alloc::vec::Vec,
    core::{
        fmt::{
            self,
            Write,
        },
        iter,
    },
    super::{
        BootServices,
        Handle,
        RuntimeServices,
        SimpleTextInputProtocol,
        SimpleTextOutputProtocol,
        Status,
        TableHeader,
        Void,
        char16,
        configuration,
        memory,
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

static mut SYSTEM_TABLE: Option<&'static mut SystemTable<'static>> = None;

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
    con_in: &'a SimpleTextInputProtocol<'a>,
    console_out_handle: Handle<'a>,
    con_out: &'a SimpleTextOutputProtocol<'a>,
    standard_error_handle: Handle<'a>,
    std_err: &'a SimpleTextOutputProtocol<'a>,
    runtime_services: &'a RuntimeServices,
    boot_services: &'a BootServices,
    configuration_tables: configuration::Tables<'a>,
}

impl SystemTable<'_> {
    pub fn allocate(&self, size: usize) -> Result<&Void, Status> {
        self.boot_services.allocate_pool(size)
    }

    pub fn deallocate(&self, pool: &Void) -> Result<(), Status> {
        self.boot_services.free_pool(pool)
    }

    pub fn memory_map(&self) -> Vec<memory::Descriptor> {
        self.boot_services.memory_map()
    }

    pub fn shutdown(&self) {
        self.runtime_services.shutdown();
    }
}

impl SystemTable<'static> {
    pub fn get() -> &'static mut Self {
        unsafe {
            SYSTEM_TABLE.as_mut().expect("Can't get a UEFI system table!")
        }
    }

    pub fn print(args: fmt::Arguments) {
        Self::get().write_fmt(args).expect("UEFI can't print.")
    }

    pub fn set(&'static mut self) {
        unsafe {
            SYSTEM_TABLE = Some(self);
        }
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

