use bitfield_struct::bitfield;

/// # HSCPARAMS1
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3.3 Structural Parameters 1 (HCSPARAMS1)
#[bitfield(u32)]
pub struct Register {
    max_slots: u8,
    #[bits(11)]
    max_intrs: u16,
    #[bits(5)]
    __: u8,
    max_ports: u8,
}

impl Register {
    pub fn number_of_ports(&self) -> usize {
        self.max_ports() as usize
    }
}

