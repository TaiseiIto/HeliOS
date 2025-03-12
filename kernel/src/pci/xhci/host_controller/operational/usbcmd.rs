use {
    bitfield_struct::bitfield,
    crate::x64,
};

/// # USB Command Register (USBCMD)
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.1 USB Command Register (USBCMD)
#[bitfield(u32)]
pub struct Register {
    rs: bool,
    hcrst: bool,
    inte: bool,
    hsee: bool,
    #[bits(3)]
    __: u8,
    lhcrst: bool,
    css: bool,
    crs: bool,
    ewe: bool,
    eu3s: bool,
    __: bool,
    cme: bool,
    ete: bool,
    tscen: bool,
    vtioen: bool,
    #[bits(15)]
    __: u16,
}

impl Register {
    pub fn is_reset(&self) -> bool {
        !self.hcrst()
    }

    pub fn reset(self) -> Self {
        self.with_hcrst(true)
    }
}
