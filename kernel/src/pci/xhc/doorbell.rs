/// # Doorbell Registers
/// ## References
/// * [eXtensible Host Controller Interface for Universal Serial Bus (xHCI)](https://www.intel.com/content/dam/www/public/us/en/documents/technical-specifications/extensible-host-controler-interface-usb-xhci.pdf) 5.6 Doorbell Registers
#[derive(Debug)]
#[repr(packed)]
pub struct Register {
    db_target: u8,
    __: u16,
    db_task_id: u16,
}

