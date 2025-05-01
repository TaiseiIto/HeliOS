use {
    alloc::vec::Vec,
    bitfield_struct::bitfield,
    core::mem,
    super::super::super::base,
};

/// # PBA Offset/PBA BIR for MSI-X
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2.4. PBA Offset/PBA BIR for MSI-X
#[bitfield(u32)]
pub struct Register {
    #[bits(3)]
    bir: u8,
    #[bits(29)]
    offset: u32,
}

impl Register {
    pub fn read(&self, index2address: &base::Index2Address, table_length: usize) -> Vec<PendingBits> {
        let bir: usize = self.bir() as usize;
        let offset: u32 = self.offset() << Self::OFFSET_OFFSET;
        let offset: usize = offset as usize;
        let pba_bits: usize = mem::size_of::<PendingBits>() * (u8::BITS as usize);
        let pba_length: usize = (table_length + pba_bits - 1) / pba_bits;
        index2address
            .get(bir)
            .unwrap()
            .offset(offset)
            .read_vector(pba_length)
    }
}

/// # Pending Bits
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) 6.8.2.10. Pending Bits for MSI-X PBA-Entries
#[derive(Debug, Default)]
pub struct PendingBits(u64);

