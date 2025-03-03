use {
    core::slice,
    super::{
        operational,
        runtime,
        super::doorbell,
    },
};

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
    pub fn doorbell_registers(&self) -> &[doorbell::Register] {
        let dboff: dboff::Register = self.dboff;
        let doorbell_array_offset: usize = dboff.get();
        let hcsparams1: hcsparams1::Register = self.hcsparams1;
        let number_of_slots: usize = hcsparams1.number_of_slots();
        let address: *const Self = self as *const Self;
        let address: usize = address as usize;
        let doorbell_registers: usize = address + doorbell_array_offset;
        let doorbell_registers: *const doorbell::Register = doorbell_registers as *const doorbell::Register;
        unsafe {
            slice::from_raw_parts(doorbell_registers, number_of_slots)
        }
    }

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

    pub fn runtime_registers(&self) -> &runtime::Registers {
        let rtsoff: rtsoff::Register = self.rtsoff;
        let runtime_register_space_offset: usize = rtsoff.get();
        let address: *const Self = self as *const Self;
        let address: usize = address as usize;
        let runtime_registers: usize = address + runtime_register_space_offset;
        let runtime_registers: *const runtime::Registers = runtime_registers as *const runtime::Registers;
        unsafe {
            &*runtime_registers
        }
    }
}

