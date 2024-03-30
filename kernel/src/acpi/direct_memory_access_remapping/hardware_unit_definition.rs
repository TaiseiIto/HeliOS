use bitfield_struct::bitfield;

/// # DMA Remapping Hardware Unit Definition Structure
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.3 DMA Remapping Hardware Unit Definition Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Structure {
    structure_type: u16,
    length: u16,
    flags: Flags,
    size: u8,
    segment_number: u16,
    register_base_address: u64,
}

impl Structure {
    pub fn length(&self) -> usize {
        self.length as usize
    }
}

#[bitfield(u8)]
struct Flags {
    include_pci_all: bool,
    #[bits(7, access = RO)]
    reserved: u8,
}

