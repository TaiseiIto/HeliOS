use bitfield_struct::bitfield;

/// # Device Context Base Address Array Pointer Register (DCBAAP)
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4.6 Device Context Base Address Array Pointer Register (DCBAAP)
#[bitfield(u64)]
pub struct Register {
    #[bits(6)]
    __: u8,
    #[bits(58)]
    dcbaap: u64,
}
