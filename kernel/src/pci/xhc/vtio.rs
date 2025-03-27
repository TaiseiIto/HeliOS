//! # VTIO Registers
//! ## References
//! * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.7 VTIO Registers

pub mod capability;
pub mod common_assignment;

#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    capability_register: capability::Register,
    common_assignment_register: common_assignment::Register,
    device_assignment_registers: [u32; 8],
    rsvd_p: [u32; 2],
    interrupter_assignment_registers: [u32; 32],
    rsvd_z: [u32; 20],
    endpoint_assignment_registers: [u32; 255],
}

