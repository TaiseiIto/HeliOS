/// # EFI System Table
/// ## References
/// * [UEFI Specification Version 2.9](https://uefi.org/sites/default/files/resources/UEFI_Spec_2_9_2021_03_18.pdf) 4.3 EFI System Table
#[derive(Debug)]
#[repr(C)]
pub struct SystemTable<'a> {
    hdr: super::TableHeader,
    firmware_vendor: super::char16::NullTerminatedString<'a>,
    firmware_revision: u32,
    console_in_handle: super::Handle<'a>,
    con_in: &'a super::SimpleTextInputProtocol<'a>,
    console_out_handle: super::Handle<'a>,
    con_out: &'a super::SimpleTextOutputProtocol<'a>,
    standard_error_handle: super::Handle<'a>,
    std_err: &'a super::SimpleTextOutputProtocol<'a>,
    runtime_services: &'a super::RuntimeServices,
    boot_services: &'a super::BootServices,
}

