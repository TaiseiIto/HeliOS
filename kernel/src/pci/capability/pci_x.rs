use {super::Header, core::fmt};

pub mod command;
pub mod status;

/// # PCI-X Capability List Item
/// ## References
/// * [PCI-X Addendum to the PCI Local Bus Specification Revision 1.0](https://www.google.com/url?sa=t&source=web&rct=j&opi=89978449&url=https://catalogue.library.cern/api/files/bd372a28-be4a-44c5-9b5a-6b793fdf2ca0/Fulltext.pdf%3Fdownload&ved=2ahUKEwif4dayjv-MAxU8SfUHHQo6Ky0QFnoECBEQAQ&usg=AOvVaw0pGLYdTmIj33uks6j01ce9) 7.2 PCI-X Capabilities List Item
#[repr(packed)]
pub struct Item {
    header: Header,
    command: command::Register,
    status: status::Register,
}

impl fmt::Debug for Item {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let command: command::Register = self.command;
        let status: status::Register = self.status;
        formatter
            .debug_struct("Item")
            .field("header", &header)
            .field("command", &command)
            .field("status", &status)
            .finish()
    }
}
