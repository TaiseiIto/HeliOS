use bitfield_struct::bitfield;

/// # VTIO Common Assignment Register
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.7.2 VTIO Common Assignment Register 1 (VTIOCA1)
#[bitfield(u32)]
pub struct Register {
    __: bool,
    crdida: bool,
    dcbaadida: bool,
    spbadida: bool,
    spbdida: bool,
    __: bool,
    icdida: bool,
    msidida: bool,
    pbdida: bool,
    dcdida: bool,
    epdida: bool,
    #[bits(21)]
    __: u32,
}
