use {
    bitfield_struct::bitfield,
    super::system_description,
};

/// # DMA Remapping Table
/// ## References
/// * [Intel Virtualization Technology for Directed I/O](https://software.intel.com/content/dam/develop/external/us/en/documents-tps/vt-directed-io-spec.pdf) 8.1 DMA Remapping Reporting Structure
#[derive(Debug)]
#[repr(packed)]
pub struct Table {
    header: system_description::Header,
    host_address_width: u8,
    flags: Flags,
    reserved0: [u8; 10],
}

impl Table {
    pub fn is_correct(&self) -> bool {
        self.header.is_correct()
    }
}

#[bitfield(u8)]
struct Flags {
    intr_remap: bool,
    x2apic_opt_out: bool,
    dma_ctrl_platform_opt_in_flag: bool,
    #[bits(5, access = RO)]
    reserved0: u8,
}

