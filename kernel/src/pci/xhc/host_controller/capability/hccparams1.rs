use bitfield_struct::bitfield;

/// # HCCPARAMS1
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3.6 Capability Parameters 1 (HCCPARAMS1)
#[bitfield(u32)]
pub struct Register {
    ac64: bool,
    bnc: bool,
    csz: bool,
    ppc: bool,
    pind: bool,
    lhrc: bool,
    ltc: bool,
    nss: bool,
    pae: bool,
    spc: bool,
    sec: bool,
    cfc: bool,
    #[bits(4)]
    max_psa_size: u8,
    xecp: u16,
}
