use bitfield_struct::bitfield;

/// # Expansion Slot Register
/// ## References
/// * [PCI-to-PCI Bridge Architecture Specification Revision 1.1 December 18, 1998](https://catalogue.library.cern/literature/d4pkk-8qm02) 3.2.6.4. Chassis Number Register
/// * [PCI-to-PCI Bridge Architecture Specification Revision 1.1 December 18, 1998](https://catalogue.library.cern/literature/d4pkk-8qm02) 13.4. The Chassis Number Register. Figure 13-2: Chassis Number Register
#[bitfield(u8)]
pub struct Register {
    chassis_number: u8,
}

