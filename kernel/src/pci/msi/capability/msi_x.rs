use {
    core::fmt,
    super::Structure as Header,
};

/// # MSI-X Capability and Table Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2. MSI-X Capability and Table Structures
#[repr(packed)]
pub struct Structure {
    header: Header,
    message_control: u16,
    table: u32,
    pba: u32,
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let capability_id: u8 = header.capability_id();
        let next_pointer: u8 = header.next_pointer();
        let message_control: u16 = self.message_control;
        let table: u32 = self.table;
        let pba: u32 = self.pba;
        formatter
            .debug_struct("Structure")
            .field("capability_id", &capability_id)
            .field("next_pointer", &next_pointer)
            .field("message_control", &message_control)
            .field("table", &table)
            .field("pba", &pba)
            .finish()
    }
}

impl<'a> From<&'a Header> for &'a Structure {
    fn from(header: &'a Header) -> Self {
        let header: *const Header = header as *const Header;
        let structure: *const Structure = header as *const Structure;
        unsafe {
            &*structure
        }
    }
}

