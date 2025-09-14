use bitfield_struct::bitfield;

/// # RTSOFF
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3.8 Runtime Register Space Offset(RTSOFF)
#[bitfield(u32)]
pub struct Register {
    #[bits(5)]
    __: u8,
    #[bits(27)]
    runtime_register_space_offset: u32,
}

impl Register {
    pub fn get(&self) -> usize {
        let runtime_register_space_offset: u32 =
            self.runtime_register_space_offset() << Self::RUNTIME_REGISTER_SPACE_OFFSET_OFFSET;
        runtime_register_space_offset as usize
    }
}
