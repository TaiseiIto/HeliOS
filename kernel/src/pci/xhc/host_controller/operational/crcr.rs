use bitfield_struct::bitfield;

/// # Command Ring Control Register (CRCR)
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.5 Command Ring Control Register (CRCR)
#[bitfield(u64)]
pub struct Register {
    rcs: bool,
    cs: bool,
    ca: bool,
    crr: bool,
    #[bits(2)]
    __: u8,
    #[bits(58)]
    command_ring_pointer: u64,
}
