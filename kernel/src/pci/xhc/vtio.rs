//! # VTIO Registers
//! ## References
//! * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.7 VTIO Registers

pub mod capability;
pub mod common_assignment;

#[derive(Debug)]
#[repr(packed)]
pub struct Registers {
    #[allow(dead_code)]
    capability_register: capability::Register,
    #[allow(dead_code)]
    common_assignment_register: common_assignment::Register,
    #[allow(dead_code)]
    device_assignment_registers: [u32; 8],
    #[allow(dead_code)]
    rsvd_p: [u32; 2],
    #[allow(dead_code)]
    interrupter_assignment_registers: [u32; 32],
    #[allow(dead_code)]
    rsvd_z: [u32; 20],
    #[allow(dead_code)]
    endpoint_assignment_registers: [u32; 255],
}

