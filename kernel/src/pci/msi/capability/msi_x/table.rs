use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    super::super::super::super::base,
};

/// # Table Offset/Table BIR for MSI-X
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2.4. Table Offset/Table BIR for MSI-X
#[bitfield(u32)]
pub struct Register {
    #[bits(3)]
    bir: u8,
    #[bits(29)]
    offset: u32,
}

impl Register {
    pub fn read(&self, index2address: &base::Index2Address, table_size: usize) -> Vec<Entry> {
        let bir: usize = self.bir() as usize;
        let offset: u32 = self.offset() << Self::OFFSET_OFFSET;
        let offset: usize = offset as usize;
        index2address
            .get(bir)
            .unwrap()
            .offset(offset)
            .read_vector(table_size)
    }
}

/// # Table Entry
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2. Fugure 6-11: MSI-X Table Structure
#[derive(Debug, Default)]
#[repr(packed)]
pub struct Entry {
    msg_addr: u64,
    msg_data: u32,
    vector_control: VectorControl,
}

/// # Vector Control
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2.9. Vector Control for MSI-X Table Entries
#[bitfield(u32)]
pub struct VectorControl {
    mask_bit: bool,
    #[bits(31)]
    __: u32,
}

