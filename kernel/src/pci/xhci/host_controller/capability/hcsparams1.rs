use bitfield_struct::bitfield;

#[bitfield(u32)]
pub struct Register {
    max_slots: u8,
    #[bits(11)]
    max_intrs: u16,
    #[bits(5)]
    __: u8,
    max_ports: u8,
}

