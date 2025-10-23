use bitfield_struct::bitfield;

/// # PMCSR_BSE - PMCSR PCI-to-PCI Bridge Support Extensions (Offset = 6)
/// ## References
/// * [PCI Power Management Interface Specification Revision 1.2](https://lekensteyn.nl/files/docs/PCI_Power_Management_12.pdf) 3.2.5. PMCSR_BSE - PMCSR PCI-to-PCI Bridge Support Extensions (Offset = 6)
#[bitfield(u8)]
pub struct Register {
    #[bits(6)]
    __: u8,
    b2_b3: bool,
    bpcc_en: bool,
}
