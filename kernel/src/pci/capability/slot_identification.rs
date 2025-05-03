use {
    core::fmt,
    super::Header,
};

/// # Slot Numbering Capabilities Register
/// ## References
/// * [PCI-to-PCI Bridge Architecture Specification Revision 1.1 December 18, 1998](https://catalogue.library.cern/literature/d4pkk-8qm02) 3.2.6. Slot Numbering Capabilities List Item. Figure 3-3: Slot Numbering Capabilities Register
#[repr(packed)]
pub struct Register {
    header: Header,
    expansion_slot: u8,
    chassis_number: u8,
}

impl fmt::Debug for Register {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header: Header = self.header.clone();
        let capability_id: u8 = header.capability_id();
        let next_pointer: u8 = header.next_pointer();
        let expansion_slot: u8 = self.expansion_slot;
        let chassis_number: u8 = self.chassis_number;
        formatter
            .debug_struct("Register")
            .field("capability_id", &capability_id)
            .field("next_pointer", &next_pointer)
            .field("expansion_slot", &expansion_slot)
            .field("chassis_number", &chassis_number)
            .finish()
    }
}

