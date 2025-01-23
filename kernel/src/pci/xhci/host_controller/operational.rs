pub mod dnctrl;
pub mod pagesize;
pub mod usbcmd;
pub mod usbsts;

/// # Host Controller Operational Registers
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4 Host Controller Operational Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    usbcmd: usbcmd::Register,
    usbsts: usbsts::Register,
    pagesize: pagesize::Register,
    __: u64,
    dnctrl: dnctrl::Register,
}

