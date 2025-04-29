pub mod pmc;
pub mod pmcsr;

use {
    core::fmt,
    super::Header,
};

/// # Power Management Register Block
/// ## References
/// * [PCI Power Management Interface Specification Revision 1.2](https://lekensteyn.nl/files/docs/PCI_Power_Management_12.pdf) 3.2. Power Management Register Block Definition
#[repr(packed)]
pub struct Registers {
    header: Header,
    pmc: pmc::Register,
    pmcsr: pmcsr::Register,
    pmcsr_bse: u8,
    data: u8,
}

impl fmt::Debug for Registers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let capability_id: u8 = header.capability_id();
        let next_pointer: u8 = header.next_pointer();
        let pmc: pmc::Register = self.pmc;
        let pmcsr: pmcsr::Register = self.pmcsr;
        let pmcsr_bse: u8 = self.pmcsr_bse;
        let data: u8 = self.data;
        formatter
            .debug_struct("Registers")
            .field("capability_id", &capability_id)
            .field("next_pointer", &next_pointer)
            .field("pmc", &pmc)
            .field("pmcsr", &pmcsr)
            .field("pmcsr_bse", &pmcsr_bse)
            .field("data", &data)
            .finish()
    }
}

