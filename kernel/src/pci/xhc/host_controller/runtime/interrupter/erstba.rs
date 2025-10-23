use bitfield_struct::bitfield;

/// # ERSTBA
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.5.2.3.2 Event Ring Segment Table Base Address Register (ERSTBA)
#[bitfield(u64)]
pub struct Register {
    #[bits(6)]
    __: u8,
    #[bits(58)]
    event_ring_segment_table_base_address_register: u64,
}
