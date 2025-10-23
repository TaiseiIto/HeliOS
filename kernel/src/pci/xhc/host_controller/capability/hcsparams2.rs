use bitfield_struct::bitfield;

/// # HSCPARAMS2
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3.4 Structural Parameters 2 (HCSPARAMS2)
#[bitfield(u32)]
pub struct Register {
    #[bits(4)]
    ist: u8,
    #[bits(4)]
    erst_max: u8,
    #[bits(13)]
    __: u16,
    #[bits(5)]
    max_scratchpad_bufs_hi: u8,
    spr: bool,
    #[bits(5)]
    max_scratchpad_bufs_lo: u8,
}
