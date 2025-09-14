use bitfield_struct::bitfield;

/// # Port Status and Control Register (PORTSC)
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.8 Port Status and Control Register (PORTSC)
#[bitfield(u32)]
pub struct Register {
    ccs: bool,
    ped: bool,
    __: bool,
    oca: bool,
    pr: bool,
    #[bits(4)]
    pls: u8,
    pp: bool,
    #[bits(4)]
    port_speed: u8,
    #[bits(2)]
    pic: u8,
    lws: bool,
    csc: bool,
    pec: bool,
    wrc: bool,
    occ: bool,
    prc: bool,
    plc: bool,
    cec: bool,
    cas: bool,
    wce: bool,
    wde: bool,
    woe: bool,
    #[bits(2)]
    __: u8,
    dr: bool,
    wpr: bool,
}
