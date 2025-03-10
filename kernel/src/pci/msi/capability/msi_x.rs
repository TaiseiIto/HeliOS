use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::fmt,
    super::{
        Header,
        super::super::base,
    },
};

pub mod pba;
pub mod table;

/// # MSI-X Capability and Table Structure
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2. MSI-X Capability and Table Structures
#[repr(packed)]
pub struct Structure {
    header: Header,
    message_control: MessageControl,
    table: table::Register,
    pba: pba::Register,
}

impl Structure {
    pub fn read_pba(&self, index2address: &base::Index2Address) -> Vec<pba::PendingBits> {
        let pba: pba::Register = self.pba;
        let table_length: usize = self.table_length();
        pba.read(index2address, table_length)
    }

    pub fn read_table(&self, index2address: &base::Index2Address) -> Vec<table::Entry> {
        let table: table::Register = self.table;
        let table_length: usize = self.table_length();
        table.read(index2address, table_length)
    }

    pub fn table_length(&self) -> usize {
        let message_control: MessageControl = self.message_control;
        let table_size: usize = message_control.table_size() as usize;
        table_size + 1
    }
}

impl fmt::Debug for Structure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let capability_id: u8 = header.capability_id();
        let next_pointer: u8 = header.next_pointer();
        let message_control: MessageControl = self.message_control;
        let table: table::Register = self.table;
        let pba: pba::Register = self.pba;
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

/// # Message Control for MSI-X
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2.3. Message Control for MSI-X
#[bitfield(u16)]
pub struct MessageControl {
    #[bits(11)]
    table_size: u16,
    #[bits(3)]
    __: u8,
    function_mask: bool,
    msi_x_enable: bool,
}

