pub mod dboff;
pub mod hccparams1;
pub mod hcsparams1;
pub mod hcsparams2;
pub mod hcsparams3;

/// # Host Controller Capability Registers
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3 Host Controller Capability Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    caplength: u8,
    __: u8,
    hciversion: u16,
    hcsparams1: hcsparams1::Register,
    hcsparams2: hcsparams2::Register,
    hcsparams3: hcsparams3::Register,
    hccparams1: hccparams1::Register,
    dboff: dboff::Register,
    rtsoff: u32,
    hccparams2: u32,
}

