use {
    bitfield_struct::bitfield,
    super::system_description,
};

/// # Windows SMM Security Mitigations Table
/// ## References
/// * [Windows SMM Security Mitigations Table](https://download.microsoft.com/download/1/8/A/18A21244-EB67-4538-BAA2-1A54E0E490B6/WSMT.docx)
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    #[allow(dead_code)]
    header: system_description::Header,
    #[allow(dead_code)]
    protection_flags: Flags,
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

/// # Protection Flags
/// ## References
/// * [Windows SMM Security Mitigations Table](https://download.microsoft.com/download/1/8/A/18A21244-EB67-4538-BAA2-1A54E0E490B6/WSMT.docx)
#[bitfield(u32)]
struct Flags {
    fixed_comm_buffers: bool,
    comm_buffer_nested_ptr_protection: bool,
    system_resource_protection: bool,
    #[bits(29)]
    __: u32,
}

