use {
    core::fmt,
    super::Header,
};

/// # APG Capabilities
/// ## References
/// * [APG V3.0 Interface Specification](http://www.playtool.com/pages/agpcompat/agp30.pdf) 2.5 AGP3.0 Programming. Figure 2-10: AGP3.0 Configuration Register Space
#[repr(packed)]
pub struct Space {
    header: Header,
    revision: u16,
    status: u32,
    command: u32,
}

impl fmt::Debug for Space {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let revision: u16 = self.revision;
        let status: u32 = self.status;
        let command: u32 = self.command;
        formatter
            .debug_struct("Space")
            .field("header", &header)
            .field("revision", &revision)
            .field("status", &status)
            .field("command", &command)
            .finish()
    }
}

