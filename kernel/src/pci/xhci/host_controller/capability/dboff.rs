use bitfield_struct::bitfield;

/// # DBOFF
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3.7 Doorbell Offset (DBOFF)
#[bitfield(u32)]
pub struct Register {
    #[bits(2)]
    __: u8,
    #[bits(30, access = RO)]
    doorbell_array_offset: u32,
}

