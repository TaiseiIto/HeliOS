/// # Host Controller Capability Registers
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3 Host Controller Capability Registers
#[repr(packed)]
pub struct Register {
    caplength: u8,
    reserved: u8,
    hciversion: u16,
    hcsparams: [u32; 3],
    hccparams1: u32,
    dboff: u32,
    rtsoff: u32,
    hccparams2: u32,
}

