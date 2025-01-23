use core::fmt;

/// # Device Notification Control Register (DNCTRL)
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.4 Device Notification Control Register (DNCTRL)
#[derive(Clone, Copy)]
pub struct Register(u32);

impl Register {
    fn n(&self, index: usize) -> bool {
        assert!(index < 0x10);
        let Self(register) = self;
        ((*register >> index) & 1) != 0
    }
}

impl fmt::Debug for Register {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_list()
            .entries((0..0x10).map(|index| self.n(index)))
            .finish()
    }
}

