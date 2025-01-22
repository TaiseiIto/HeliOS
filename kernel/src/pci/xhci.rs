//! # XHCI (eXtensible Host Controller Interface)
//! ## References
//! * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf)

use super::{
    Function,
    class,
};

pub mod host_controller;

#[derive(Debug)]
pub struct Registers {
    address: usize,
}

impl TryFrom<&Function> for Registers {
    type Error = ();

    fn try_from(function: &Function) -> Result<Self, Self::Error> {
        (function.class_code() == class::Code::UsbXhci)
            .then(|| function.memory_address())
            .flatten()
            .map(|address| Self {
                address,
            })
            .ok_or(())
    }
}

