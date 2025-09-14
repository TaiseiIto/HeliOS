/// # VTIO Capability Register
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.7.1 VTIO Capability Register (VTIOCAP)
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Register {
    #[allow(dead_code)]
    pdmaid: u16,
    #[allow(dead_code)]
    admaid: u16,
}
