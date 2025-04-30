/// # IMOD
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.5.2.2 Interrupter Moderation Register (IMOD)
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Register {
    #[allow(dead_code)]
    interval: u16,
    #[allow(dead_code)]
    counter: u16,
}

