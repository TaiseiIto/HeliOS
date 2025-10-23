use bitfield_struct::bitfield;

/// # VPD Address
/// ## References
/// * [PCI Local Bus Specification Revision 3.0](https://lekensteyn.nl/files/docs/PCI_SPEV_V3_0.pdf) I. Vital Product Data. Figure I-1: VPD Capability Structure
#[bitfield(u16)]
pub struct Register {
    #[bits(15)]
    address: u16,
    f: bool,
}

impl Register {
    pub fn read_address(address: u16) -> Self {
        Self::default().with_address(address).with_f(false)
    }

    pub fn can_read_data(&self) -> bool {
        self.f()
    }
}
