use bitfield_struct::bitfield;

/// # HCCPARAMS2
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3.9 Capability Parameters 2 (HCCPARAMS2)
#[bitfield(u32)]
pub struct Register {
    #[bits(access = RO)]
    u3c: bool,
    #[bits(access = RO)]
    cmc: bool,
    #[bits(access = RO)]
    fsc: bool,
    #[bits(access = RO)]
    ctc: bool,
    #[bits(access = RO)]
    lec: bool,
    #[bits(access = RO)]
    cic: bool,
    #[bits(access = RO)]
    etc: bool,
    #[bits(access = RO)]
    tsc: bool,
    #[bits(access = RO)]
    gsc: bool,
    #[bits(access = RO)]
    vtc: bool,
    #[bits(22)]
    __: u32,
}
