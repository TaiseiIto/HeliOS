use bitfield_struct::bitfield;

/// # USB Status Register (USBSTS)
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.2 USB Status Register (USBSTS)
#[bitfield(u32)]
pub struct Register {
    hch: bool,
    __: bool,
    hse: bool,
    eint: bool,
    pcd: bool,
    #[bits(3)]
    __: u8,
    sss: bool,
    rss: bool,
    sre: bool,
    cnr: bool,
    hce: bool,
    #[bits(19)]
    __: u32,
}

impl Register {
    pub fn is_halted(&self) -> bool {
        self.hch()
    }

    pub fn is_ready(&self) -> bool {
        !self.cnr()
    }
}

