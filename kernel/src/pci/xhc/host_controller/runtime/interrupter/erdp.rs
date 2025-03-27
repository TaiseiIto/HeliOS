use bitfield_struct::bitfield;

/// # ERDP
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.5.2.3.3 Event Ring Deque Pointer Register (ERDP)
#[bitfield(u64)]
pub struct Register {
    #[bits(3)]
    desi: u8,
    ehb: bool,
    #[bits(60)]
    event_ring_deque_pointer: u64,
}

