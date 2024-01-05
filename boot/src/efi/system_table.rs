use super::{
    BootServices,
    Handle,
    RuntimeServices,
    SimpleTextInputProtocol,
    SimpleTextOutputProtocol,
    TableHeader,
    char16,
    configuration,
};

static mut SYSTEM_TABLE: Option<&'static SystemTable<'static>> = None;

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
    pub fn shutdown(&self) {
        self.runtime_services.shutdown();
    }
}

impl SystemTable<'static> {
    pub fn get() -> &'static Self {
        unsafe {
            SYSTEM_TABLE.expect("Can't get a UEFI system table!")
        }
    }

    pub fn set(&'static self) {
        unsafe {
            SYSTEM_TABLE = Some(self);
        }
    }
}

