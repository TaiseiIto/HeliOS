use super::operational;

pub mod dboff;
pub mod hccparams1;
pub mod hccparams2;
pub mod hcsparams1;
pub mod hcsparams2;
pub mod hcsparams3;
pub mod rtsoff;
pub mod vtiosoff;

/// # Host Controller Capability Registers
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.3 Host Controller Capability Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    caplength: u8,
    __: u8,
    hciversion: u16,
    hcsparams1: hcsparams1::Register,
    hcsparams2: hcsparams2::Register,
    hcsparams3: hcsparams3::Register,
    hccparams1: hccparams1::Register,
    dboff: dboff::Register,
    rtsoff: rtsoff::Register,
    hccparams2: hccparams2::Register,
    vtiosoff: vtiosoff::Register,
}

impl Registers {
    pub fn number_of_ports(&self) -> usize {
        let hcsparams1: hcsparams1::Register = self.hcsparams1;
        hcsparams1.number_of_ports()
    }

    pub fn operational_registers(&self) -> &operational::Registers {
        let caplength: u8 = self.caplength;
        let caplength: usize = caplength as usize;
        let address: *const Self = self as *const Self;
        let address: usize = address as usize;
        let operational_registers: usize = address + caplength;
        let operational_registers: *const operational::Registers = operational_registers as *const operational::Registers;
        unsafe {
            &*operational_registers
        }
    }
}

