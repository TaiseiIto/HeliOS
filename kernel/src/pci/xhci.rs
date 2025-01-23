//! # XHCI (eXtensible Host Controller Interface)
//! ## References
//! * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf)

use {
    alloc::vec::Vec,
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
    fn capability_registers(&self) -> &host_controller::capability::Registers {
        let Self {
            address,
        } = self;
        let capability_register: *const host_controller::capability::Registers = *address as *const host_controller::capability::Registers;
        unsafe {
            &*capability_register
        }
    }

    fn operational_registers(&self) -> &host_controller::operational::Registers {
        self.capability_registers()
            .operational_registers()
    }

    fn ports(&self) -> Vec<&host_controller::operational::port::Registers> {
        let number_of_ports: usize = self
            .capability_registers()
            .number_of_ports();
        let operational_registers: &host_controller::operational::Registers = self.operational_registers();
        (1..=number_of_ports)
            .map(|port| operational_registers.port_registers(port))
            .collect()
    }
}

impl fmt::Debug for Registers {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Registers")
            .field("capability_registers", self.capability_registers())
            .field("operational_registers", self.operational_registers())
            .field("ports", &self.ports())
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

