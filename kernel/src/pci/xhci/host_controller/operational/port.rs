/// # Host Controller Operational Port Registers
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.8 Port Status and Control Register (PORTSC)
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.9 Port PM Statis and Control Register (PORTPMSC)
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.10 Port Link Info Register (PORTLI)
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.11 Port Hardware LPM Control Register (PORTHLPMC)
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    sc: u32,
    pmsc: u32,
    li: u32,
    hlpmc: u32,
}

impl Registers {
    pub const OFFSET: usize = 0x400;
}

