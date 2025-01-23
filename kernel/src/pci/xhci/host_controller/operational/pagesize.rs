/// # USB Status Register (USBSTS)
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.2 USB Status Register (USBSTS)
#[derive(Clone, Copy, Debug)]
#[repr(packed)]
pub struct Register {
    page_size: u16,
    __: u16,
}

