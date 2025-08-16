use {
    core::fmt,
    super::Header,
};

pub mod command;
pub mod revision;
pub mod status;

/// # AGP Capabilities
/// ## References
/// * [AGP V3.0 Interface Specification](http://www.playtool.com/pages/agpcompat/agp30.pdf) 2.5 AGP3.0 Programming. Figure 2-10: AGP3.0 Configuration Register Space
#[repr(packed)]
pub struct Space {
    header: Header,
    revision: revision::Register,
    status: status::Register,
    command: command::Register,
}

impl fmt::Debug for Space {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let revision: revision::Register = self.revision;
        let status: status::Register = self.status;
        let command: command::Register = self.command;
        formatter
            .debug_struct("Space")
            .field("header", &header)
            .field("revision", &revision)
            .field("status", &status)
            .field("command", &command)
            .finish()
    }
}

