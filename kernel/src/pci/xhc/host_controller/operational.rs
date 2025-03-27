use {
    core::mem,
    crate::x64,
};

pub mod config;
pub mod crcr;
pub mod dcbaap;
pub mod dnctrl;
pub mod pagesize;
pub mod port;
pub mod usbcmd;
pub mod usbsts;

/// # Host Controller Operational Registers
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.4 Host Controller Operational Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    usbcmd: usbcmd::Register,
    usbsts: usbsts::Register,
    pagesize: pagesize::Register,
    _0: u64,
    dnctrl: dnctrl::Register,
    crcr: crcr::Register,
    _1: u128,
    dcbaap: dcbaap::Register,
    config: config::Register,
}

impl Registers {
    pub fn port_registers(&self, port: usize) -> &port::Registers {
        assert!(1 <= port);
        let address: *const Self = self as *const Self;
        let address: usize = address as usize;
        let port: usize = address + port::Registers::OFFSET + (port - 1) * mem::size_of::<port::Registers>();
        let port: *const port::Registers = port as *const port::Registers;
        unsafe {
            &*port
        }
    }

    pub fn reset(&mut self) {
        while !self.usbsts().is_halted() {
            x64::pause();
        }
        self.usbcmd = self.usbcmd().reset();
        while !self.usbcmd().is_reset() {
            x64::pause();
        }
        while !self.usbsts().is_ready() {
            x64::pause();
        }
    }

    pub fn usbsts(&self) -> usbsts::Register {
        self.usbsts
    }

    pub fn usbcmd(&self) -> usbcmd::Register {
        self.usbcmd
    }
}

