use bitfield_struct::bitfield;

/// # Expansion Slot Register
/// ## References
/// * [PCI-to-PCI Bridge Architecture Specification Revision 1.1 December 18, 1998](https://catalogue.library.cern/literature/d4pkk-8qm02) 3.2.6.3. Expansion Slot Register. Table 3-10: Expansion Slot Register
/// * [PCI-to-PCI Bridge Architecture Specification Revision 1.1 December 18, 1998](https://catalogue.library.cern/literature/d4pkk-8qm02) 13.3. The Slot Number Register. Figure 13-1: Expansion Slot Register
#[bitfield(u8)]
pub struct Register {
    #[bits(5)]
    expansion_slots_provided: u8,
    first_in_chassis: bool,
    #[bits(2)]
    __: u8,
}

