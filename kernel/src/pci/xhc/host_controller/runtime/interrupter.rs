pub mod iman;
pub mod imod;
pub mod erstsz;
pub mod erstba;
pub mod erdp;

/// # Interrupter Register Set
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.5.2 Interrupter Register Set
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct RegisterSet {
    iman: iman::Register,
    imod: imod::Register,
    erstsz: erstsz::Register,
    __: u32,
    erstba: erstba::Register,
    erdp: erdp::Register,
}

