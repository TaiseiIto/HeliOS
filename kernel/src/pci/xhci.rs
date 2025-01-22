//! # XHCI (eXtensible Host Controller Interface)
//! ## References
//! * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf)

use {
    core::fmt,
    super::{
        Function,
        class,
    },
};

pub mod host_controller;

pub struct Registers {
    address: usize,
}

impl Registers {
    fn host_controller_capability_registers(&self) -> &host_controller::capability::Registers {
        let Self {
            address,
        } = self;
        let host_controller_capability_register: *const host_controller::capability::Registers = *address as *const host_controller::capability::Registers;
        unsafe {
            &*host_controller_capability_register
        }
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Registers")
            .field("host_controller_capability_registers", self.host_controller_capability_registers())
            .finish()
    }
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

