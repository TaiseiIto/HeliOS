use bitfield_struct::bitfield;

/// # Configure Register (CONFIG)
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.7 Configure Register (CONFIG)
#[bitfield(u32)]
pub struct Register {
    max_slots_en: u8,
    u3e: bool,
    cie: bool,
    #[bits(22)]
    __: u32,
}

