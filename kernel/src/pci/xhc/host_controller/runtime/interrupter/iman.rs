use bitfield_struct::bitfield;

/// # IMAN
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.5.2.1 Interrupt Management Register (IMAN)
#[bitfield(u32)]
pub struct Register {
    ip: bool,
    ie: bool,
    #[bits(30)]
    __: u32,
}
