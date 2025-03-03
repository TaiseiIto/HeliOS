//! # VTIO Registers
//! ## References
//! * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.7 VTIO Registers

pub mod capability;
pub mod common_assignment;

#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    capability: capability::Register,
    common_assignment: common_assignment::Register,
}

