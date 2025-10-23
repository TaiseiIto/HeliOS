use bitfield_struct::bitfield;

/// # MFINDEX
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.5.1 Microframe Index Register (MFINDEX)
#[bitfield(u32)]
pub struct Register {
    #[bits(14)]
    microframe_index: u16,
    #[bits(18)]
    __: u32,
}
