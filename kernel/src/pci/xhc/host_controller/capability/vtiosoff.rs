use bitfield_struct::bitfield;

/// # VTIOSOFF
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3.10 Virtualication Based Trusted IO Register Space Offset (VTIOSOFF)
#[bitfield(u32)]
pub struct Register {
    #[bits(12)]
    __: u16,
    #[bits(20, access = RO)]
    vtio_register_space_offset: u32,
}

impl Register {
    pub fn get(&self) -> usize {
        let vtio_register_space_offset: u32 =
            self.vtio_register_space_offset() << Self::VTIO_REGISTER_SPACE_OFFSET_OFFSET;
        vtio_register_space_offset as usize
    }
}
