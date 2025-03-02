/// # Host Controller Runtime Registers
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.5 Host Controller Runtime Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    mfindex: u32,
}

