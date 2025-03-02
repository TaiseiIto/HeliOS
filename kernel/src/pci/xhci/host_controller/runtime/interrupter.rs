/// # Interrupter Register Set
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.5.2 Interrupter Register Set
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct RegisterSet {
    iman: u32,
    imod: u32,
    erstsz: u32,
    __: u32,
    erstba: u64,
    erdb: u64,
}

